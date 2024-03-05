use libds3::prelude::*;
use practice_tool_core::key::Key;
use practice_tool_core::widgets::nudge_position::NudgePosition;
use practice_tool_core::widgets::Widget;

use crate::widgets::position::SavePosition;

pub(crate) fn nudge_position(
    ptr: (PointerChain<f32>, PointerChain<[f32; 3]>),
    nudge: f32,
    key_nudge_up: Option<Key>,
    key_nudge_down: Option<Key>,
) -> Box<dyn Widget> {
    Box::new(NudgePosition::new(SavePosition::new(ptr, nudge), key_nudge_up, key_nudge_down))
}
