use imgui::{ColorStackToken, StyleColor, Ui};

// #FF9330
pub const ORANGE: [f32; 4] = [1., 0.576, 0.188, 1.];
// #884100
pub const DARK_ORANGE: [f32; 4] = [0.533, 0.255, 0., 1.];
// #BBBBBB
pub const GRAY: [f32; 4] = [0.733, 0.733, 0.733, 1.];
// #333333
pub const DARK_GRAY: [f32; 4] = [0.2, 0.2, 0.2, 1.];

pub(crate) enum StyleState {
    ActiveValid,
    ActiveInvalid,
    InactiveValid,
    InactiveInvalid,
}

impl StyleState {
    pub(crate) fn get_style_token<'a>(&self, ui: &'a Ui) -> ColorStackToken<'a> {
        ui.push_style_color(
            StyleColor::Text,
            match self {
                StyleState::ActiveValid => ORANGE,
                StyleState::ActiveInvalid => DARK_ORANGE,
                StyleState::InactiveValid => GRAY,
                StyleState::InactiveInvalid => DARK_GRAY,
            },
        )
    }
}
