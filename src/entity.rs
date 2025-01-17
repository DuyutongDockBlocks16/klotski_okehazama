use hecs::{Entity, World};

pub struct WallDuringGame {}

pub struct BlockDuringGame {}

pub struct ExitDuringGame {
    passable_by: Vec<BlockType>,
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

pub enum BlockType {
    Regular,   // 普通棋子
    Special,   // 可以通过 Exit 的特殊棋子
}

pub struct BlockEscapeType {
    pub block_type: BlockType,
}

pub fn create_floor(world: &mut World, position: PositionDuringGame) -> Entity {
    world.spawn((
        PositionDuringGame { z: 5, ..position },
        Renderable {
            path: "/images/floor.png".to_string(),
        },
    ))
}

pub fn create_exit(world: &mut World, position: PositionDuringGame) -> Entity {
    world.spawn((
        PositionDuringGame { z: 6, ..position },
        Renderable {
            path: "/images/exit.png".to_string(),
        },
        ExitDuringGame{ 
            passable_by: vec![BlockType::Special],
        }
    ))
}

pub fn create_block(
    world: &mut World,
    position: PositionDuringGame,
    block_id: &str,
    size: Option<&(u8, u8, bool)>
) -> Entity {

    let (width, height, can_escapse) = match size {
        Some(&(w, h, can_escapse)) => (w, h, can_escapse), // 解构 Some 并提取 w 和 h
        None => (0, 0, false),        // 提供默认值
    };

    let mut occupied_cells=vec![];

    for i in 0..width{
        for j in 0..height{
            occupied_cells.push(
                PositionDuringGame{
                    x: position.x + i,
                    y: position.y - j,
                    z: 10,
                }
            )
        }
    }

    let block_type = match can_escapse {
        false => BlockType::Regular,
        true => BlockType::Special
    };

    world.spawn((
        PositionDuringGame { z: 10, ..position },
        Renderable {
            path: "/images/".to_string() + block_id + ".png",
        },
        BlockId {
            block_id: block_id.to_string(),
        },
        Size { width, height },
        CollisionVolume {
            occupied_cells
        },
        BlockEscapeType {
            block_type
        },
        BlockDuringGame {},
        Movable {},
    ))
}

pub fn create_wall(world: &mut World, position: PositionDuringGame) -> Entity {
    world.spawn((
        PositionDuringGame { z: 10, ..position },
        Renderable {
            path: "/images/mountain.png".to_string(),
        },
        WallDuringGame {},
        Immovable {},
    ))
}

