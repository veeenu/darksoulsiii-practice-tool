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
/// Begin, one per `Phase`, end.
const TICKS: usize = 6;

#[derive(Clone, Copy)]
struct Sample {
    frame: u32,
    ui_state: &'static str,
    /// Performance counter ticks, indexed by `Phase`. Zero for marks that
    /// weren't recorded this frame.
    ticks: [i64; TICKS],
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
            current: Sample { frame: 0, ui_state: "", ticks: [0; TICKS] },
            buf: Vec::with_capacity(BUFFER_LEN),
            full_tx,
            empty_rx,
        }))
    }

    #[inline(always)]
    pub(crate) fn begin(&mut self) {
        if let Some(r) = &mut self.0 {
            r.current.ticks = [0; TICKS];
            r.current.ticks[0] = qpc();
        }
    }

    #[inline(always)]
    pub(crate) fn mark(&mut self, phase: Phase) {
        if let Some(r) = &mut self.0 {
            r.current.ticks[phase as usize] = qpc();
        }
    }

    #[inline(always)]
    pub(crate) fn end(&mut self, ui_state: &'static str) {
        if let Some(r) = &mut self.0 {
            r.current.ticks[TICKS - 1] = qpc();
            r.current.ui_state = ui_state;
            if !r.commit() {
                self.0 = None;
            }
        }
    }
}

impl Recorder {
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
    writeln!(
        out,
        "frame,ui_state,start_ns,interval_ns,hotkeys_ns,xinput_ns,radial_ns,ui_ns,logs_ns,total_ns"
    )?;
    out.flush()?;

    let mut base = None;
    let mut prev_start = None;

    for mut batch in full_rx {
        for Sample { frame, ui_state, ticks } in batch.iter().copied() {
            let (t0, tn) = (ticks[0], ticks[TICKS - 1]);
            let base = *base.get_or_insert(t0);
            let interval = prev_start.map(|prev| t0 - prev).unwrap_or(0);
            prev_start = Some(t0);

            write!(out, "{frame},{ui_state},{},{}", to_ns(t0 - base), to_ns(interval))?;

            // Each phase runs from the previous recorded mark; unrecorded marks
            // are left empty.
            let mut prev = t0;
            for &t in &ticks[1..] {
                if t == 0 {
                    write!(out, ",")?;
                } else {
                    write!(out, ",{}", to_ns(t - prev))?;
                    prev = t;
                }
            }

            writeln!(out, ",{}", to_ns(tn - t0))?;
        }
        out.flush()?;

        // Fails only once the recorder is gone; keep draining its final batch.
        batch.clear();
        empty_tx.send(batch).ok();
    }

    Ok(())
}
