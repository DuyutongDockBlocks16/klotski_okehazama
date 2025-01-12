use crate::board::Board;

pub struct Game {
    board_with_blocks: Board
}
impl Game {
    pub fn new(board_with_blocks: Board) -> Self {
        Game {
            board_with_blocks
        }
    }

    pub fn authorize_game_blocks_amount(&self) -> bool{
        if self.board_with_blocks.blocks.is_empty() {
            return false;
        }
        return true;
    }

    pub fn authorize_game_blocks_location_conflict(&self) -> (bool, String){
        let return_message = String::from("");



        let no_conflict_message = "No location conflict";
        let return_message = return_message + no_conflict_message;
        return (true, return_message)
    }
}