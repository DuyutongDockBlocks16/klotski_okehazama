use specs::{Component, NullStorage, VecStorage, World, WorldExt};


pub struct WallDuringGame {}

pub struct BlockDuringGame {}

pub struct ExitDuringGame {
    // passable_by: Vec<BlockType>,
}


// ANCHOR: components_movement
pub struct Movable;

pub struct Immovable;

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