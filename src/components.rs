use crate::events::Event;
use std::fmt;
use std::fmt::Display;
use std::time::Duration;
use ggez::audio;
use ggez::audio::SoundSource;
use ggez::Context;
use std::collections::HashMap;

pub struct WallDuringGame {}

pub struct BlockDuringGame {}

pub struct ExitDuringGame {
    // passable_by: Vec<BlockType>,
}


// ANCHOR: components_movement
pub struct Movable;

pub struct Immovable;

#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct PositionDuringGame {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Debug)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

pub struct BlockId{
    pub block_id: String,
}


#[derive(Debug)]
pub struct CollisionVolume {
    pub occupied_cells: Vec<PositionDuringGame>,
}

pub struct Renderable {
    pub path: String,
}

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Regular,   // 普通棋子
    Special,   // 可以通过 Exit 的特殊棋子
}

pub struct BlockEscapeType {
    pub block_type: BlockType,
}

#[derive(Default)]
pub enum GameplayState {
    #[default]
    Playing,
    Won,
}


impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, std::boxed::Box<audio::Source>>,
}

impl AudioStore {
    pub fn play_sound(&mut self, context: &mut Context, sound: &str) {
        if let Some(source) = self.sounds.get_mut(sound) {
            if source.play_detached(context).is_ok() {
                println!("Playing sound: {}", sound);
            }
        }
    }
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}
