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
    paths: Vec<String>,
}

pub enum RenderableKind {
    Static,
    Animated,
}

impl Renderable {
    pub fn new_static(path: &str) -> Self {
        Self {
            paths: vec![path.to_string()],
        }
    }

    pub fn new_animated(paths: Vec<&str>) -> Self {
        Self {
            paths: paths.iter().map(|p| p.to_string()).collect(),
        }
    }

    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn path(&self, path_index: usize) -> String {
        // If we get asked for a path that is larger than the
        // number of paths we actually have, we simply mod the index
        // with the length to get an index that is in range.
        self.paths[path_index % self.paths.len()].clone()
    }
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
