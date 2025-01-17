use std::collections::HashMap;
use hecs::{Entity, World};

use ggez::{
    graphics::{self, DrawParam, Image},
    input::keyboard::KeyCode,
    Context,
};

use std::sync::Mutex;
use lazy_static::lazy_static;
use glam::Vec2;

pub static mut MAP_WIDTH: u8 = 0;
pub static mut MAP_HEIGHT: u8 = 0;
const TILE_WIDTH: f32 = 100.0;

use crate::entity::{*};

lazy_static! {
    static ref SELECTED_BLOCK_ID: Mutex<String> = Mutex::new(String::new());
}

fn set_selected_block_id(id: String) {
    let mut block_id = SELECTED_BLOCK_ID.lock().unwrap();
    *block_id = id.to_string();
}

fn get_selected_block_id() -> String {
    let block_id = SELECTED_BLOCK_ID.lock().unwrap();
    block_id.clone()
}

pub unsafe fn select_block( context: &mut Context) {

    if get_selected_block_id() != ""{
        return;
    }

    // 跟踪是否正在选择数字

    // 获取用户输入的 block_id
    let selected_id = if context.keyboard.is_key_just_pressed(KeyCode::Key0) {
        println!("0 pressed!");
        0
    } else if context.keyboard.is_key_pressed(KeyCode::Key1) {
        1
    } else if context.keyboard.is_key_pressed(KeyCode::Key2) {
        2
    } else {
        return;
    };


    println!("selected_id: {:?}", selected_id);

    set_selected_block_id(selected_id.to_string());

}

// todo
pub unsafe fn move_block(world: &mut World, context: &mut Context) {
    let mut to_move: Vec<(Entity, KeyCode)> = Vec::new();

    // get all the movables and immovables
    let mov: HashMap<(u8, u8), Entity> = world
        .query::<(&PositionDuringGame, &Movable)>()
        .iter()
        .map(|t| ((t.1 .0.x, t.1 .0.y), t.0))
        .collect::<HashMap<_, _>>();

    let mov_collision_volume: HashMap<(u8, u8), Entity> = world
        .query::<(&CollisionVolume)>()
        .iter()
        .flat_map(|(entity, collision_volume)|{
            collision_volume.occupied_cells.iter().map(move |pos| ((pos.x, pos.y), entity))
        })
        .collect::<HashMap<_, _>>();

    let immov: HashMap<(u8, u8), Entity> = world
        .query::<(&PositionDuringGame, &Immovable)>()
        .iter()
        .map(|t| ((t.1 .0.x, t.1 .0.y), t.0))
        .collect::<HashMap<_, _>>();

    for (_, (position, block_id, size, block_escape_type)) in 
    world.query::<(&mut PositionDuringGame, &BlockId, &Size, &BlockEscapeType)>().iter() {
        if block_id.block_id == get_selected_block_id() {
            if context.keyboard.is_key_repeated() {
                continue;
            }

            // Now iterate through current position to the end of the map
            // on the correct axis and check what needs to move.
            let key = if context.keyboard.is_key_pressed(KeyCode::Up) {
                KeyCode::Up
            } else if context.keyboard.is_key_pressed(KeyCode::Down) {
                KeyCode::Down
            } else if context.keyboard.is_key_pressed(KeyCode::Left) {
                println!("move left!");
                KeyCode::Left
            } else if context.keyboard.is_key_pressed(KeyCode::Right) {
                KeyCode::Right
            } else {
                continue;
            };
            
            let (start, end, is_x) = match key {
                KeyCode::Up => (position.y, 0, false),
                KeyCode::Down => (position.y, MAP_HEIGHT - 1, false),
                KeyCode::Left => (position.x, 0, true),
                KeyCode::Right => (position.x, MAP_WIDTH - 1, true),
                _ => continue,
            };

            let range = if start < end {
                (start..=end).collect::<Vec<_>>()
            } else {
                (end..=start).rev().collect::<Vec<_>>()
            };


            if (key == KeyCode::Up || key == KeyCode::Down) && size.width > 1 {

                let width = size.width;

                let mut can_not_move_flag = false;

                for i in 0..width {
                    
                    if can_not_move_flag == true {
                        break;
                    }

                    for x_or_y in &range {
                        let pos = (position.x + i, *x_or_y);
                        match mov.get(&pos) {
                            Some(entity) => {
                                let entity_to_check = (*entity, key); // 要检查的元素
                                if !to_move.iter().any(|&x| x == entity_to_check) {
                                    to_move.push(entity_to_check); // 如果不存在相同元素，则添加
                                } 
                            },
                            None => {
                                match mov_collision_volume.get(&pos){
                                    Some(entity) => {
                                        let entity_to_check = (*entity, key); // 要检查的元素
                                        if !to_move.iter().any(|&x| x == entity_to_check) {
                                            to_move.push(entity_to_check); // 如果不存在相同元素，则添加
                                        } 
                                    },
                                    None => {
                                        match immov.get(&pos) {
                                            Some(_id) => {
                                                to_move.clear();
                                                can_not_move_flag = true;
                                            },
                                            None => break,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

            } else if (key == KeyCode::Left || key == KeyCode::Down) && size.height > 1 {
                let height = size.height;

                let mut can_not_move_flag = false;


                for i in 0..height {

                    if can_not_move_flag == true {
                        break;
                    }


                    for x_or_y in &range {
                        let pos = (*x_or_y, position.y + i);

                        // find a movable
                        // if it exists, we try to move it and continue
                        // if it doesn't exist, we continue and try to find an immovable instead
                        match mov.get(&pos) {
                            Some(entity) => {
                                let entity_to_check = (*entity, key); // 要检查的元素
                                if !to_move.iter().any(|&x| x == entity_to_check) {
                                    to_move.push(entity_to_check); // 如果不存在相同元素，则添加
                                } 
                            },
                            None => {
                                match mov_collision_volume.get(&pos){
                                    Some(entity) => {
                                        let entity_to_check = (*entity, key); // 要检查的元素
                                        if !to_move.iter().any(|&x| x == entity_to_check) {
                                            to_move.push(entity_to_check); // 如果不存在相同元素，则添加
                                        } 
                                    },
                                    None => {
                                        match immov.get(&pos) {
                                            Some(_id) => {
                                                to_move.clear();
                                                can_not_move_flag = true;
                                            },
                                            None => break,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

            } else {

                // single_cell_block_move_desicion(key, to_move, position, mov_collision_volume, immov);
    
                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };
    
                    // find a movable
                    // if it exists, we try to move it and continue
                    // if it doesn't exist, we continue and try to find an immovable instead
                    match mov.get(&pos) {
                        Some(entity) => to_move.push((*entity, key)),
                        None => {
                            match mov_collision_volume.get(&pos){
                                Some(entity) => {
                                    let entity_to_check = (*entity, key); // 要检查的元素
                                    if !to_move.iter().any(|&x| x == entity_to_check) {
                                        to_move.push(entity_to_check); // 如果不存在相同元素，则添加
                                    } 
                                },
                                None => {
                                    match immov.get(&pos) {
                                        Some(_id) => to_move.clear(),
                                        None => break,
                                    }
                                }
                            }
                        }
                    }
                }
                
            }
        }
    }

    // Now actually move what needs to be moved
    for (entity, key) in to_move {
        let mut position = world.get::<&mut PositionDuringGame>(entity).unwrap();
        let mut volume = world.get::<&mut CollisionVolume>(entity).unwrap();
        
        match key {
            KeyCode::Up => {
                position.y -= 1;
                for cell in volume.occupied_cells.iter_mut() {
                    cell.y -= 1; // 更新 occupied_cells 中的 y 坐标
                }
            },
            KeyCode::Down => {
                position.y += 1;
                for cell in volume.occupied_cells.iter_mut() {
                    cell.y += 1; // 更新 occupied_cells 中的 y 坐标
                }
            },
            KeyCode::Left => {
                position.x -= 1;
                for cell in volume.occupied_cells.iter_mut() {
                    cell.x -= 1; // 更新 occupied_cells 中的 x 坐标
                }
            },
            KeyCode::Right => {
                position.x += 1;
                for cell in volume.occupied_cells.iter_mut() {
                    cell.x += 1; // 更新 occupied_cells 中的 x 坐标
                }
            },
            _ => (),
        }
    }
}

// fn single_cell_block_move_desicion(key: KeyCode, to_move: Vec<(Entity, KeyCode)>, position: &mut PositionDuringGame, mov: Vec<(Entity, KeyCode)>, ) {
//     let (start, end, is_x) = match key {
//         KeyCode::Up => (position.y, 0, false),
//         KeyCode::Down => (position.y, MAP_HEIGHT - 1, false),
//         KeyCode::Left => (position.x, 0, true),
//         KeyCode::Right => (position.x, MAP_WIDTH - 1, true),
//         _ => continue,
//     };

//     let range = if start < end {
//         (start..=end).collect::<Vec<_>>()
//     } else {
//         (end..=start).rev().collect::<Vec<_>>()
//     };

//     for x_or_y in range {
//         let pos = if is_x {
//             (x_or_y, position.y)
//         } else {
//             (position.x, x_or_y)
//         };

//         // find a movable
//         // if it exists, we try to move it and continue
//         // if it doesn't exist, we continue and try to find an immovable instead
//         match mov.get(&pos) {
//             Some(entity) => to_move.push((*entity, key)),
//             None => {
//                 match mov_collision_volume.get(&pos){
//                     Some(entity) => {
//                         let entity_to_check = (*entity, key); // 要检查的元素
//                         if !to_move.iter().any(|&x| x == entity_to_check) {
//                             to_move.push(entity_to_check); // 如果不存在相同元素，则添加
//                         } 
//                     },
//                     None => {
//                         match immov.get(&pos) {
//                             Some(_id) => to_move.clear(),
//                             None => break,
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

pub fn run_rendering(world: &World, context: &mut Context) {
    // Clearing the screen (this gives us the background colour)
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));

    // Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut query = world.query::<(&PositionDuringGame, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&PositionDuringGame, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    // Iterate through all pairs of positions & renderables, load the image
    // and draw it at the specified position.
    for (_, (position, renderable)) in rendering_data.iter() {
        // Load the image
        let image = Image::from_path(context, renderable.path.clone()).unwrap();
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        // draw
        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        canvas.draw(&image, draw_params);
    }

    // Finally, present the canvas, this will actually display everything
    // on the screen.
    canvas.finish(context).expect("expected to present");
}
