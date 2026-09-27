//! The render thread only reads the performance counter and copies a sample
//! into a preallocated buffer. Full buffers are handed to a low priority writer
//! thread, which formats them into a CSV file and hands them back: the render
//! thread never allocates, formats, or touches the disk.

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::{mem, thread};

use hudhook::tracing::{error, info};
use practice_tool_core::crossbeam_channel::{self, Receiver, Sender, TryRecvError};
use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
use windows::Win32::System::Threading::{
    GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_LOWEST,
};

use super::Phase;
use crate::util;

const BUFFER_LEN: usize = 512;
const BUFFER_COUNT: usize = 4;
/// Maximum marks per frame; any further marks are dropped.
const MAX_MARKS: usize = 16;

#[derive(Clone, Copy)]
struct Sample {
    frame: u32,
    ui_state: &'static str,
    /// Performance counter ticks at the start of the frame.
    start: i64,
    /// Number of marks recorded this frame.
    len: u8,
    /// Marks in execution order: the phase each one ends, and its ticks.
    phases: [u8; MAX_MARKS],
    ticks: [i64; MAX_MARKS],
}

pub(crate) struct Profiler(Option<Recorder>);

struct Recorder {
    current: Sample,
    buf: Vec<Sample>,
    full_tx: Sender<Vec<Sample>>,
    empty_rx: Receiver<Vec<Sample>>,
}

impl Profiler {
    fn disabled() -> Self {
        Profiler(None)
    }

    /// Starts the writer thread, which creates (truncating) the output file.
    pub(crate) fn new() -> Self {
        let Some(path) = util::get_dll_path().map(|mut path| {
            path.pop();
            path.push("jdsd_dsiii_practice_tool.profile.csv");
            path
        }) else {
            error!("Profiler disabled: could not construct output file path");
            return Self::disabled();
        };

        let mut freq = 0i64;
        if unsafe { QueryPerformanceFrequency(&mut freq) }.is_err() || freq <= 0 {
            error!("Profiler disabled: could not query performance counter frequency");
            return Self::disabled();
        }

        let (full_tx, full_rx) = crossbeam_channel::bounded(BUFFER_COUNT);
        let (empty_tx, empty_rx) = crossbeam_channel::bounded(BUFFER_COUNT);
        for _ in 1..BUFFER_COUNT {
            empty_tx.send(Vec::with_capacity(BUFFER_LEN)).ok();
        }

        let spawned = thread::Builder::new().name("profiler-writer".into()).spawn(move || {
            if let Err(e) = write_samples(&path, freq, full_rx, empty_tx) {
                error!("Profiler writer for {path:?} stopped: {e:?}");
            }
        });

        if let Err(e) = spawned {
            error!("Profiler disabled: could not spawn writer thread: {e:?}");
            return Self::disabled();
        }

        info!("Profiler enabled");

        Profiler(Some(Recorder {
            current: Sample {
                frame: 0,
                ui_state: "",
                start: 0,
                len: 0,
                phases: [0; MAX_MARKS],
                ticks: [0; MAX_MARKS],
            },
            buf: Vec::with_capacity(BUFFER_LEN),
            full_tx,
            empty_rx,
        }))
    }

    #[inline(always)]
    pub(crate) fn begin(&mut self) {
        if let Some(r) = &mut self.0 {
            r.current.len = 0;
            r.current.start = qpc();
        }
    }

    #[inline(always)]
    pub(crate) fn mark(&mut self, phase: Phase) {
        if let Some(r) = &mut self.0 {
            r.mark(phase);
        }
    }

    /// Marks the end of `Phase::Logs` and records the frame.
    #[inline(always)]
    pub(crate) fn end(&mut self, ui_state: &'static str) {
        if let Some(r) = &mut self.0 {
            r.mark(Phase::Logs);
            r.current.ui_state = ui_state;
            if !r.commit() {
                self.0 = None;
            }
        }
    }
}

impl Recorder {
    #[inline(always)]
    fn mark(&mut self, phase: Phase) {
        let Sample { len, phases, ticks, .. } = &mut self.current;
        let i = *len as usize;
        if i < MAX_MARKS {
            ticks[i] = qpc();
            phases[i] = phase as u8;
            *len += 1;
        }
    }

    /// Returns `false` if the writer thread is gone and recording should stop.
    #[inline(always)]
    fn commit(&mut self) -> bool {
        // Never reallocates: the buffer is handed off as soon as it's full.
        self.buf.push(self.current);
        self.current.frame = self.current.frame.wrapping_add(1);

        if self.buf.len() < BUFFER_LEN {
            true
        } else {
            self.hand_off()
        }
    }

    #[cold]
    #[inline(never)]
    fn hand_off(&mut self) -> bool {
        match self.empty_rx.try_recv() {
            Ok(empty) => self.full_tx.try_send(mem::replace(&mut self.buf, empty)).is_ok(),
            Err(TryRecvError::Empty) => {
                self.buf.clear();
                true
            },
            Err(TryRecvError::Disconnected) => false,
        }
    }
}

impl Drop for Recorder {
    fn drop(&mut self) {
        if !self.buf.is_empty() {
            self.full_tx.try_send(mem::take(&mut self.buf)).ok();
        }
    }
}

#[inline(always)]
fn qpc() -> i64 {
    let mut ticks = 0i64;
    // Cannot fail on Windows XP and later.
    unsafe { QueryPerformanceCounter(&mut ticks).ok() };
    ticks
}

fn write_samples(
    path: &Path,
    freq: i64,
    full_rx: Receiver<Vec<Sample>>,
    empty_tx: Sender<Vec<Sample>>,
) -> io::Result<()> {
    unsafe { SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_LOWEST).ok() };

    let to_ns = |ticks: i64| (ticks as i128 * 1_000_000_000 / freq as i128) as i64;

    let mut out = BufWriter::new(File::create(path)?);
    write!(out, "frame,ui_state,start_ns,interval_ns")?;
    for name in Phase::NAMES {
        write!(out, ",{name}_ns")?;
    }
    writeln!(out, ",total_ns")?;
    out.flush()?;

    let mut base = None;
    let mut prev_start = None;

    for mut batch in full_rx {
        for Sample { frame, ui_state, start, len, phases, ticks } in batch.iter().copied() {
            let base = *base.get_or_insert(start);
            let interval = prev_start.map(|prev| start - prev).unwrap_or(0);
            prev_start = Some(start);

            // Each mark ends its phase at the time elapsed since the previous
            // mark. Phases marked more than once are summed; unmarked ones are
            // left empty.
            let mut durations = [None::<i64>; Phase::COUNT];
            let mut prev = start;
            for (&phase, &t) in phases.iter().zip(&ticks).take(len as usize) {
                let duration = &mut durations[phase as usize];
                *duration = Some(duration.unwrap_or(0) + t - prev);
                prev = t;
            }

            write!(out, "{frame},{ui_state},{},{}", to_ns(start - base), to_ns(interval))?;
            for duration in durations {
                match duration {
                    Some(d) => write!(out, ",{}", to_ns(d))?,
                    None => write!(out, ",")?,
                }
            }
            writeln!(out, ",{}", to_ns(prev - start))?;
        }
        out.flush()?;

        // Fails only once the recorder is gone; keep draining its final batch.
        batch.clear();
        empty_tx.send(batch).ok();
    }

    Ok(())
}
