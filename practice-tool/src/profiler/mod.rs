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

/// Marks recorded within a frame, each ending the phase it names. A phase
/// lasts from the previous mark, in execution order, to its own; phases can
/// happen in any order, or not at all, depending on the UI state.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) enum Phase {
    /// Font selection and display/hide hotkeys.
    Hotkeys,
    /// Reading the controller state for the radial menu. Not recorded when the
    /// radial menu is disabled.
    XInput,
    /// Rest of the radial menu handling.
    Radial,
    /// Forcing the mouse cursor visible while the menu is open, which writes
    /// game memory every frame.
    CursorShow,
    /// Opening the tool's window, plus the header, buttons and popups when the
    /// menu is closed.
    UiSetup,
    /// Indicators (IGT, position, ...), shown when the menu is closed.
    Indicators,
    /// Widgets' `render` (menu open) or `render_closed` (menu closed).
    WidgetsRender,
    /// Widgets' hotkey handling (`interact`).
    WidgetsInteract,
    /// Everything left in the UI state: the Close/Eject buttons and closing
    /// the window.
    UiFinish,
    /// Collecting widget logs and drawing the log window. Ends the frame.
    Logs,
}

#[allow(dead_code)]
impl Phase {
    pub(crate) const COUNT: usize = Phase::Logs as usize + 1;

    /// CSV column names, indexed by `Phase`.
    pub(crate) const NAMES: [&'static str; Phase::COUNT] = [
        "hotkeys",
        "xinput",
        "radial",
        "cursor_show",
        "ui_setup",
        "indicators",
        "widgets_render",
        "widgets_interact",
        "ui_finish",
        "logs",
    ];
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
