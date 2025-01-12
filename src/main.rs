use crate::board::{Board, ExitSide};

mod block;
mod board;
mod game;

fn initialize_box() -> Board {
    let height:u8 = 5;
    let width:u8 = 4;
    let distance_to_edge:u8 = 1;
    let length:u8 = 2;
    let side: ExitSide = ExitSide::Bottom;

    let exit_position = board::ExitPosition {
        side,
        distance_to_edge,
        length,
    };

    Board::new(width, height, exit_position)
}

fn main() {
    // println!("Hello, world!");

    let mut board = initialize_box();
    let mut imagawa_block = block::Block::new(
        0,
        "Imagawa Yoshimoto",
        "今川義元",
        2,
        2,
        (1, 5),
        true
    );

    let mut oda_bloc = block::Block::new(
        1,
        "Oda Nobunaga",
        "織田信長",
        2,
        1,
        (1, 3),
        false
    );



    board.add_block(imagawa_block);
    board.add_block(oda_bloc);

    let mut game = game::Game::new(board);

    if !game.authorize_game_blocks_amount() {
        println!("The amount of blocks should more than 0!")
    }

    let (authorization_passed_flag, return_message) = game.authorize_game_blocks_location_conflict();

    if !authorization_passed_flag {
        println!("{}", return_message);
    }

}


