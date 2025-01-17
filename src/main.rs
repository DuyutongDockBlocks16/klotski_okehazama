use crate::board::{Board, ExitSide};
use std::fs::File;
use std::io::Read;
use crate::block::Block;

mod block;
mod board;
mod game;
mod entity;
mod rendering;


fn initialize_box() -> Board {
    let height:u8 = 5;
    let width:u8 = 4;
    let side: ExitSide = ExitSide::Bottom;
    let distance_to_edge:u8 = 1;
    let length:u8 = 2;

    let exit_position = board::ExitPosition {
        side,
        distance_to_edge,
        length,
    };

    Board::new(width, height, exit_position)
}

fn main() -> std::io::Result<()> {
    // println!("Hello, world!");

    let mut board = initialize_box();

    let mut file = File::open("./src/blocks.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let blocks: Vec<Block> = serde_json::from_str(&contents).expect("Failed to parse JSON");

    // 将读取的 Block 实例添加到 Board
    for block in blocks {
        board.add_block(block);
    }

    board.display();

    let mut game = game::Game::new(board);

    if game.authorize_game_exit_location() {
        if game.authorize_game_blocks_amount(){
            let (authorization_passed_flag, return_message) = game.authorize_game_blocks_location_conflict();
            if authorization_passed_flag {
                unsafe { game.initialize(); }
                game.display();
                game.start().expect("Game Internal Error");

                // game.over();
            } else { println!("{}", return_message); }
        } else { println!("The amount of blocks should more than 0!") }
    } else { println!("Please check the configuration of exit position") }

    Ok(())
}


