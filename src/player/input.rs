use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::Player;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[non_exhaustive]
pub enum PlayerAction {
    Move,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerAction::Move => InputControlKind::DualAxis,
        }
    }
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad and keyboard input bindings
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::WASD);

        input_map
    }
}

pub fn handle_actions(query: Query<&ActionState<PlayerAction>, With<Player>>) {
    let Some(action) = query.iter().next() else {
        return;
    };

    // todo
}
