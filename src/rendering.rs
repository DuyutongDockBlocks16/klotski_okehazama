use std::collections::HashMap;
use hecs::{Entity, World};
use itertools::Itertools;
use crate::events::*;
use ggez::{
    graphics::{self, Canvas, Color, DrawParam, Image, PxScale, Text, TextFragment},
    input::keyboard::KeyCode,
    Context,
};


use std::sync::Mutex;
use lazy_static::lazy_static;
use glam::Vec2;
use std::time::Duration;

use crate::constants::{EXIT_KEY, EXIT_POSITIONS, MAP_WIDTH, MAP_HEIGHT, TILE_WIDTH};


use crate::components::*;

lazy_static! {
    static ref SELECTED_BLOCK_ID: Mutex<String> = Mutex::new(String::new());
}

fn set_selected_block_id(id: String) {
    let mut block_id = SELECTED_BLOCK_ID.lock().unwrap();
    *block_id = id;
}

fn get_selected_block_id() -> String {
    let block_id = SELECTED_BLOCK_ID.lock().unwrap();
    block_id.clone()
}

pub unsafe fn select_block(world: &mut World, context: &mut Context) {

    if get_selected_block_id() != ""{
        return;
    }

    // 跟踪是否正在选择数字

    // 获取用户输入的 block_id
    let selected_id = if context.keyboard.is_key_just_pressed(KeyCode::Key0) {
        println!("0 pressed!");
        0
    } else if context.keyboard.is_key_pressed(KeyCode::Key1) {
        println!("1 pressed!");
        1
    } else if context.keyboard.is_key_pressed(KeyCode::Key2) {
        println!("2 pressed!");
        2
    } else if context.keyboard.is_key_pressed(KeyCode::Key3) {
        println!("3 pressed!");
        3
    } else if context.keyboard.is_key_pressed(KeyCode::Key4) {
        println!("4 pressed!");
        4
    } else if context.keyboard.is_key_pressed(KeyCode::Key5) {
        println!("5 pressed!");
        5
    } else if context.keyboard.is_key_pressed(KeyCode::Key6) {
        println!("6 pressed!");
        6
    } else if context.keyboard.is_key_pressed(KeyCode::Key7) {
        println!("7 pressed!");
        7
    } else if context.keyboard.is_key_pressed(KeyCode::Key8) {
        println!("8 pressed!");
        8
    } else if context.keyboard.is_key_pressed(KeyCode::Key9) {
        println!("9 pressed!");
        9
    } else {
        return;
    };
    


    println!("selected_id: {:?}", selected_id);

    set_selected_block_id(selected_id.to_string());
    let mut query = world.query::<&mut Gameplay>();
    let gameplay = query.iter().next().unwrap().1;
    gameplay.seleced_block_id = selected_id.to_string();


}

// todo
pub unsafe fn move_block(world: &mut World, context: &mut Context) {

    if get_selected_block_id() == ""{
        return;
    }

    let mut to_move: Vec<(Entity, KeyCode)> = Vec::new();
    let mut events = Vec::new();

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

    for (current_entity, (position, block_id, size, block_escape_type)) in 
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
                // println!("move left!");
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
                                                events.push(Event::BlockHitObstacle {});
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
                                                events.push(Event::BlockHitObstacle {});
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
                                        print!("push");
                                        to_move.push(entity_to_check); // 如果不存在相同元素，则添加
                                    } 
                                },
                                None => {
                                    match immov.get(&pos) {
                                        Some(_id) => {
                                            to_move.clear();
                                            events.push(Event::BlockHitObstacle {});
                                            break;
                                        },
                                        None => break,
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // 处理可以逃出棋子的case
            if block_escape_type.block_type == BlockType::Special && EXIT_KEY == key && to_move.is_empty() {
                let mut exit_adjacenct_positions :Vec<PositionDuringGame>= vec![];

                match key {
                    KeyCode::Down => {
                        for i in 0..size.width {
                            exit_adjacenct_positions.push(
                                PositionDuringGame{
                                    x: position.x + i,
                                    y: position.y + size.height,
                                    z: 6 as u8,
                                }
                            );
                        }
                    },
                    KeyCode::Up => {
                        for i in 0..size.width {
                            exit_adjacenct_positions.push(
                                PositionDuringGame{
                                    x: position.x + i,
                                    y: position.y - 1,
                                    z: 6 as u8,
                                }
                            );
                        }
                    },
                    KeyCode::Left => {
                        for i in 0..size.height {
                            exit_adjacenct_positions.push(
                                PositionDuringGame{
                                    x: position.x - 1,
                                    y: position.y + i,
                                    z: 6 as u8,
                                }
                            );
                        }
                    },
                    KeyCode::Right => {
                        for i in 0..size.height {
                            exit_adjacenct_positions.push(
                                PositionDuringGame{
                                    x: position.x + size.width,
                                    y: position.y + i,
                                    z: 6 as u8,
                                }
                            );
                        }
                    },
                    _ => ()
                }

                println!("exit_adjacenct_positions: {:?}",exit_adjacenct_positions);

                let mut all_exit_positions_occupied = true;
                
                for ele in exit_adjacenct_positions {

                    match EXIT_POSITIONS.contains(&ele) {
                        true => {
                            let entity_to_check = (current_entity, key); // 要检查的元素
                            if !to_move.iter().any(|&x| x == entity_to_check) {
                                to_move.push(entity_to_check); // 如果不存在相同元素，则添加
                            } 
                        },
                        false => {
                            all_exit_positions_occupied = false;
                            to_move.clear();
                            break;
                        },
                    }
                }

                if all_exit_positions_occupied == true {
                    let mut query = world.query::<&mut Gameplay>();
                    let gameplay = query.iter().next().unwrap().1;
                    gameplay.state = GameplayState::Won;
                }
                
            }

            let selected_id = "";
            
            set_selected_block_id(selected_id.to_string());
            let mut query = world.query::<&mut Gameplay>();
            let gameplay = query.iter().next().unwrap().1;
            gameplay.seleced_block_id = selected_id.to_string();
            println!("get_selected_block_id: {:?}", get_selected_block_id());

        }
    }

    // Update gameplay moves
    if !to_move.is_empty() {
        let mut query = world.query::<&mut Gameplay>();
        let gameplay = query.iter().next().unwrap().1;
        gameplay.moves_count += 1;
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

                // set_selected_block_id("".to_string());

            },
            KeyCode::Down => {
                position.y += 1;
                for cell in volume.occupied_cells.iter_mut() {
                    cell.y += 1; // 更新 occupied_cells 中的 y 坐标
                }

                // set_selected_block_id("".to_string());
            },
            KeyCode::Left => {
                position.x -= 1;
                for cell in volume.occupied_cells.iter_mut() {
                    cell.x -= 1; // 更新 occupied_cells 中的 x 坐标
                }

                // set_selected_block_id("".to_string());
            },
            KeyCode::Right => {
                position.x += 1;
                for cell in volume.occupied_cells.iter_mut() {
                    cell.x += 1; // 更新 occupied_cells 中的 x 坐标
                }

                // set_selected_block_id("".to_string());
            },
            _ => (),
        }

        events.push(Event::BlockMoved(BlockMoved { entity }));

    }

    

    {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut events);
    }

}

pub fn run_rendering(world: &World, context: &mut Context) {
    // Clearing the screen (this gives us the background colour)
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));
    // ANCHOR_END: run_rendering

    // Get time
    let mut query = world.query::<&Time>();
    let time = query.iter().next().unwrap().1;

    // Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut query = world.query::<(&PositionDuringGame, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&PositionDuringGame, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z);

    // ANCHOR: rendering_batches
    let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

    // Iterate each of the renderables, determine which image path should be rendered
    // at which drawparams, and then add that to the rendering_batches.
    for (_, (position, renderable)) in rendering_data.iter() {
        // Load the image
        let image_path = get_image(renderable, time.delta);
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;
        let z = position.z;

        // draw
        let draw_param = DrawParam::new().dest(Vec2::new(x, y));
        rendering_batches
            .entry(z)
            .or_default()
            .entry(image_path)
            .or_default()
            .push(draw_param);
    }
    // ANCHOR_END: rendering_batches

    // ANCHOR: rendering_batches_2
    // Iterate spritebatches ordered by z and actually render each of them
    for (_z, group) in rendering_batches
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
    {
        for (image_path, draw_params) in group {
            let image = Image::from_path(context, image_path).unwrap();
            let mut mesh_batch = graphics::InstanceArray::new(context, Some(image));

            for draw_param in draw_params.iter() {
                mesh_batch.push(*draw_param);
            }

            canvas.draw(&mesh_batch, graphics::DrawParam::new());
        }
    }
    // ANCHOR_END: rendering_batches_2

    // Render any text
    let mut query = world.query::<&Gameplay>();
    let gameplay = query.iter().next().unwrap().1;

    draw_text(&mut canvas, &format!("Status: {}", &gameplay.state.to_string()), 620.0, 80.0);
    draw_text(&mut canvas, &format!("Move Count: {}", &gameplay.moves_count.to_string()), 620.0, 100.0);

    // ANCHOR: render_fps
    // Render FPS
    let fps = format!("FPS: {:.0}", context.time.fps());
    draw_text(&mut canvas, &fps, 620.0, 120.0);
    // ANCHOR_END: render_fps

    draw_text(&mut canvas, &format!("Selected Block: {}", &gameplay.seleced_block_id.to_string()), 620.0, 140.0);

    let instructions = [
        "Save Imagawa Yoshimoto!",
        "",
        "Step into the world of strategy and history with this captivating",
        "rendition of the classic Klotski puzzle!",
        "",
        "Your mission: rescue Imagawa Yoshimoto, who is trapped in the",
        "narrow passes of Okehazama, and guide him safely to",
        "Okazaki Castle.",
        "",
        "How to Play:",
        "Each block in the puzzle has a number displayed in its top-left corner.",
        "Use the number keys (0–9) above the letter keys on your keyboard",
        "to select the block you wish to move.",
        "After selecting a block, use the arrow keys (↑, ↓, ←, →) to",
        "navigate the block in the desired direction.",
        "Repeat the process until Imagawa Yoshimoto reaches the exit.",
        "",
        "Win Condition:",
        "Achieve victory by moving Imagawa Yoshimoto to the exit.",
        "It's time to rewrite history!",
        "",
        "Ready to Rewrite History? TRY HARD!",
    ];

    let mut y = 180.0; // Starting y-coordinate
    for line in &instructions {
        draw_text(&mut canvas, line, 620.0, y); // Adjust x and y positions as needed
        y += 20.0; // Increment y-coordinate for the next line
    }


    // ANCHOR: run_rendering_end
    // Finally, present the canvas, this will actually display everything
    // on the screen.
    canvas.finish(context).expect("expected to present");
}

pub fn draw_text(canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
    let text = Text::new(TextFragment {
        text: text_string.to_string(),
        color: Some(Color::new(0.0, 0.0, 0.0, 1.0)),
        scale: Some(PxScale::from(20.0)),
        ..Default::default()
    });

    canvas.draw(&text, Vec2::new(x, y));
}


pub fn get_image(renderable: &Renderable, delta: Duration) -> String {
    let path_index = match renderable.kind() {
        RenderableKind::Static => {
            // We only have one image, so we just return that
            0
        }
        RenderableKind::Animated => {
            // If we have multiple, we want to select the right one based on the delta time.
            // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
            // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
            // we technically are on the next iteration of the loop (or on 0), but we will let
            // the renderable handle this logic of wrapping frames.
            ((delta.as_millis() % 1000) / 250) as usize
        }
    };

    renderable.path(path_index)
}