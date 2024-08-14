use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[non_exhaustive]
pub enum PlayerActionTopDown {
    Move,
}

impl Actionlike for PlayerActionTopDown {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::DualAxis,
        }
    }
}

impl PlayerActionTopDown {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad and keyboard input bindings
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::WASD);

        input_map
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[non_exhaustive]
pub enum PlayerActionSidescroller {
    Move,
    Jump,
}

impl Actionlike for PlayerActionSidescroller {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            Self::Move => InputControlKind::Axis,
            _ => InputControlKind::Button,
        }
    }
}

impl PlayerActionSidescroller {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad and keyboard input bindings
        input_map.insert_axis(Self::Move, GamepadControlAxis::LEFT_X);
        input_map.insert_axis(Self::Move, KeyboardVirtualAxis::AD);

        input_map.insert(Self::Jump, KeyCode::Space);
        input_map.insert(Self::Jump, GamepadButtonType::South);

        input_map
    }
}
