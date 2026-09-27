//! Low-overhead frame timing for the render loop, enabled by the `profiling`
//! feature. Samples are written to `jdsd_dsiii_practice_tool.profile.csv` next
//! to the DLL.
//!
//! Without the feature, `Profiler` is a zero-sized type whose methods are empty
//! and compile away entirely.

#[cfg(feature = "profiling")]
mod recorder;

#[cfg(feature = "profiling")]
pub(crate) use recorder::Profiler;

/// Intermediate timestamps recorded within a frame. Each phase is measured
/// from the previous recorded mark.
#[allow(dead_code)]
pub(crate) enum Phase {
    /// Font selection and display/hide hotkeys.
    Hotkeys = 1,
    /// Reading the controller state for the radial menu. Not recorded when the
    /// radial menu is disabled.
    XInput = 2,
    /// Rest of the radial menu handling.
    Radial = 3,
    /// Widget/indicator rendering for the current UI state.
    Ui = 4,
}

#[cfg(not(feature = "profiling"))]
pub(crate) struct Profiler;

#[cfg(not(feature = "profiling"))]
impl Profiler {
    pub(crate) fn new() -> Self {
        Profiler
    }

    #[inline(always)]
    pub(crate) fn begin(&mut self) {}

    #[inline(always)]
    pub(crate) fn mark(&mut self, _phase: Phase) {}

    #[inline(always)]
    pub(crate) fn end(&mut self, _ui_state: &'static str) {}
}
