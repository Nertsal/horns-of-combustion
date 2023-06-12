use super::*;

#[derive(Debug)]
pub enum PlayerAction {
    Shoot { target_pos: Position },
    SwitchState,
}
