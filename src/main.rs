use crate::board::{Board, ExitSide};

mod block;
mod board;

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

    Board::new(height, width, exit_position)
}

fn main() {
    // println!("Hello, world!");

    let mut board = initialize_box();
    let mut imagawa_block = block::Block::new(
        "Imagawa Yoshimoto",
        "今川義元",
        2,
        2,
        (1, 5),
        true
    );

    let mut oda_bloc = block::Block::new(
        "Oda Nobunaga",
        "織田信長",
        2,
        1,
        (1, 3),
        false
    );

    board.display();

    imagawa_block.display();

    board.add_block(imagawa_block);
    board.add_block(oda_bloc)

}


