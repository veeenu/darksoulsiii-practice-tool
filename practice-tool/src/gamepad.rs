use std::sync::atomic::{AtomicU64, Ordering};

use windows::Win32::UI::Input::XboxController::{XINPUT_GAMEPAD_BUTTON_FLAGS, XINPUT_STATE};

/// Lock-free snapshot of a controller's state, written by the XInput hook on
/// the game's input thread and read by the render loop.
///
/// Only the fields the tool uses (buttons, triggers and left stick) are kept,
/// packed into a single atomic so that a reader can never observe a torn
/// state. The right stick and packet number read back as zero.
pub(crate) struct GamepadState(AtomicU64);

impl GamepadState {
    pub(crate) const fn new() -> Self {
        GamepadState(AtomicU64::new(0))
    }

    /// Stores `state`, or a neutral state (nothing pressed) if `None`.
    #[inline]
    pub(crate) fn store(&self, state: Option<&XINPUT_STATE>) {
        let packed = state.map_or(0, |state| {
            let g = &state.Gamepad;
            g.wButtons.0 as u64
                | (g.bLeftTrigger as u64) << 16
                | (g.bRightTrigger as u64) << 24
                | (g.sThumbLX as u16 as u64) << 32
                | (g.sThumbLY as u16 as u64) << 48
        });

        // No other data is published alongside the value, so no ordering is
        // needed beyond the atomicity of the store itself.
        self.0.store(packed, Ordering::Relaxed);
    }

    #[inline]
    pub(crate) fn load(&self) -> XINPUT_STATE {
        let packed = self.0.load(Ordering::Relaxed);

        let mut state = XINPUT_STATE::default();
        state.Gamepad.wButtons = XINPUT_GAMEPAD_BUTTON_FLAGS(packed as u16);
        state.Gamepad.bLeftTrigger = (packed >> 16) as u8;
        state.Gamepad.bRightTrigger = (packed >> 24) as u8;
        state.Gamepad.sThumbLX = (packed >> 32) as u16 as i16;
        state.Gamepad.sThumbLY = (packed >> 48) as u16 as i16;
        state
    }
}

#[cfg(test)]
mod tests {
    use windows::Win32::UI::Input::XboxController::XINPUT_GAMEPAD;

    use super::*;

    #[test]
    fn test_round_trip() {
        let gamepad_state = GamepadState::new();
        assert_eq!(gamepad_state.load(), XINPUT_STATE::default());

        for (lx, ly) in
            [(0, 0), (i16::MIN, i16::MAX), (i16::MAX, i16::MIN), (-1, 1), (-12345, 6789)]
        {
            let state = XINPUT_STATE {
                dwPacketNumber: 42,
                Gamepad: XINPUT_GAMEPAD {
                    wButtons: XINPUT_GAMEPAD_BUTTON_FLAGS(0xF3FF),
                    bLeftTrigger: 0xAB,
                    bRightTrigger: 0xFF,
                    sThumbLX: lx,
                    sThumbLY: ly,
                    sThumbRX: 1000,
                    sThumbRY: -1000,
                },
            };

            gamepad_state.store(Some(&state));
            let loaded = gamepad_state.load();

            assert_eq!(loaded.dwPacketNumber, 0);
            assert_eq!(loaded.Gamepad.wButtons, state.Gamepad.wButtons);
            assert_eq!(loaded.Gamepad.bLeftTrigger, state.Gamepad.bLeftTrigger);
            assert_eq!(loaded.Gamepad.bRightTrigger, state.Gamepad.bRightTrigger);
            assert_eq!(loaded.Gamepad.sThumbLX, lx);
            assert_eq!(loaded.Gamepad.sThumbLY, ly);
            assert_eq!((loaded.Gamepad.sThumbRX, loaded.Gamepad.sThumbRY), (0, 0));
        }

        gamepad_state.store(None);
        assert_eq!(gamepad_state.load(), XINPUT_STATE::default());
    }
}
