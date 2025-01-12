use crate::board::Board;
use std::collections::HashMap;

pub struct Game {
    pub board_with_blocks: Board,

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
        let mut return_message = String::from("");
        let mut authorization_passed_flag:bool = true;
        let mut occupied_grids:Vec<(u8, u8, String)> = vec![];

        // 首先检查是否有棋子摆放超出边界
        self.board_with_blocks.blocks.iter().for_each(|block| {
            if block.current_location.0 < 0 {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the left border;\n", block.block_english_name)
                );
            }

            if block.current_location.0 > self.board_with_blocks.width - 1 {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the right border;\n", block.block_english_name)
                );
            }

            if block.current_location.1 < 1 {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the bottom border;\n", block.block_english_name)
                );
            }

            if block.current_location.1 > self.board_with_blocks.height {
                authorization_passed_flag = false;
                // println!("{:?}",block.current_location);
                // println!("{:?}",self.board_with_blocks.height);
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the top border;\n", block.block_english_name)
                );
            }

            for i in 0..block.width {
                for j in 0..block.height {
                    occupied_grids.push((block.current_location.0+i,block.current_location.1-j,block.block_english_name.to_string()))
                }
            }


        });

        let mut grid_map: HashMap<(u8, u8), Vec<String>> = HashMap::new();

        for (x, y, block_name) in occupied_grids {
            grid_map.entry((x, y)).or_insert_with(Vec::new).push(block_name);
        }

        // 查找被多个 Block 占用的格子
        for (grid, blocks) in &grid_map {
            if blocks.len() > 1 {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!(
                        "Grid ({}, {}) is occupied by: {:?}; \n",
                        grid.0,
                        grid.1,
                        blocks
                    )
                );
            }
        }

        return (authorization_passed_flag, return_message)
    }
}