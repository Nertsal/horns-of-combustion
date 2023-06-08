use super::*;

#[derive(Debug)]
pub enum PlayerAction {
    Shoot { target_pos: vec2<Coord> },
}
