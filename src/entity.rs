use hecs::{Entity, World};

pub struct WallDuringGame {}

pub struct BlockDuringGame {}

pub struct ExitDuringGame {}

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
        ExitDuringGame{}
    ))
}

pub fn create_block(
    world: &mut World,
    position: PositionDuringGame,
    block_id: &str,
    size: Option<&(u8, u8)>
) -> Entity {

    let (width, height) = match size {
        Some(&(w, h)) => (w, h), // 解构 Some 并提取 w 和 h
        None => (0, 0),        // 提供默认值
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

