use hecs::{Entity, World};
use crate::components::*;

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
                    y: position.y + j,
                    z: 10,
                }
            )
        }
    }

    let block_type = match can_escapse {
        false => BlockType::Regular,
        true => BlockType::Special
    };

    // 打印棋子的相关信息
    println!("Creating block with ID: {}", block_id);
    println!("Block Type: {:?}", block_type);
    println!("Occupied Cells:");
    for cell in &occupied_cells {
        println!(" - Position: ({}, {}, {})", cell.x, cell.y, cell.z);
    }
    println!("==============================================");

    world.spawn((
        PositionDuringGame { z: 9, ..position },
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


pub fn create_exit(world: &mut World, position: PositionDuringGame) -> Entity {
    world.spawn((
        PositionDuringGame { z: 6, ..position },
        Renderable {
            path: "/images/exit.png".to_string(),
        },
        ExitDuringGame{},
        Immovable {},
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

pub fn create_floor(world: &mut World, position: PositionDuringGame) -> Entity {
    world.spawn((
        PositionDuringGame { z: 5, ..position },
        Renderable {
            path: "/images/floor.png".to_string(),
        },
    ))
}

pub fn create_gameplay(world: &mut World) -> Entity {
    world.spawn((Gameplay::default(),))
}

pub fn create_time(world: &mut World) -> Entity {
    world.spawn((Time::default(),))
}

pub fn create_event_queue(world: &mut World) -> Entity {
    world.spawn((EventQueue::default(),))
}

pub fn create_audio_store(world: &mut World) -> Entity {
    world.spawn((AudioStore::default(),))
}