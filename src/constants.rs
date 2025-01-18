use ggez::input::keyboard::KeyCode;
use crate::components::PositionDuringGame;
use crate::game::GameState;

pub static mut EXIT_POSITIONS: Vec<PositionDuringGame> = vec![];
pub static mut EXIT_KEY: KeyCode = KeyCode::Down;
pub static mut GAME_STATE: GameState = GameState::Running;
pub static mut MAP_WIDTH: u8 = 0;
pub static mut MAP_HEIGHT: u8 = 0;
pub const TILE_WIDTH: f32 = 100.0;