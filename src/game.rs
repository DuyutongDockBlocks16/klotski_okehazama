use crate::board::{Board, ExitSide};
use std::collections::HashMap;
use serde::de::Unexpected::Option;
use crate::block::Block;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

pub struct Exit{
    adjacent_grid: Vec<Position>,
    exit_direction: ExitSide,
}

pub struct BlockInGame {
    pub block_id : u8,
    pub block_english_name: String,   // 英文名称
    pub block_japanese_name: String, // 日文名称
    pub width: u8,                   // 宽度
    pub height: u8,                  // 高度
    pub current_location: Position, // 当前位置(x,y)
    pub can_move_up: bool,          // 是否可以向上移动
    pub can_move_down: bool,          // 是否可以向下移动
    pub can_move_left: bool,          // 是否可以向左移动
    pub can_move_right: bool,          // 是否可以向右移动
    pub can_escape: bool,            // 是否可以逃脱
}

pub struct Game {
    pub board_with_blocks: Board,
    pub blocks_in_game : Vec<BlockInGame>,
    pub grid: Vec<Vec<Option<BlockInGame>>>,
    pub exit: Exit,
}

impl Game {
    pub fn new(board_with_blocks: Board) -> Self {
        Game {
            board_with_blocks,
            blocks_in_game: vec![],
            grid: vec![],
            exit: Exit { adjacent_grid: vec![], exit_direction: ExitSide::Bottom },
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
            if block.initial_location.0 < 0 {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the left border;\n", block.block_english_name)
                );
            }

            if block.initial_location.0 > self.board_with_blocks.width {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the right border;\n", block.block_english_name)
                );
            }

            if block.initial_location.1 < 0 {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the bottom border;\n", block.block_english_name)
                );
            }

            if block.initial_location.1 > self.board_with_blocks.height {
                authorization_passed_flag = false;
                // println!("{:?}",block.current_location);
                // println!("{:?}",self.board_with_blocks.height);
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the top border;\n", block.block_english_name)
                );
            }

            for i in 0..block.width {
                for j in 0..block.height {
                    occupied_grids.push((block.initial_location.0+i,block.initial_location.1-j,block.block_english_name.to_string()))
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

    pub fn authorize_game_exit_location(&self) -> bool{
        let mut authorization_passed_flag:bool = true;

        if self.board_with_blocks.exit_position.side == ExitSide::Bottom
            || self.board_with_blocks.exit_position.side == ExitSide::Top {
            if self.board_with_blocks.exit_position.distance_to_edge + self.board_with_blocks.exit_position.length
                > self.board_with_blocks.width {
                authorization_passed_flag = false;
            }
        } else {
            if self.board_with_blocks.exit_position.distance_to_edge + self.board_with_blocks.exit_position.length
                > self.board_with_blocks.height{
                authorization_passed_flag = false;
            }
        }

        return authorization_passed_flag;
    }

    fn is_reach_the_top_bounds(&self, position: Position) -> bool {
        return position.y == self.board_with_blocks.height-1;
    }

    fn is_reach_the_bottom_bounds(&self, position: Position) -> bool {
        return position.y == 0;
    }

    fn is_reach_the_right_bounds(&self, position: Position) -> bool {
        return position.x == self.board_with_blocks.width-1;
    }

    fn is_reach_the_left_bounds(&self, position: Position) -> bool {
        return position.x == 0;
    }

    fn is_top_position_empty(&self, position: Position) -> bool {
        self.grid[position.y + 1][position.x].is_none()
    }

    fn is_bottom_position_empty(&self, position: Position) -> bool {
        self.grid[position.y - 1][position.x].is_none()
    }

    fn is_right_position_empty(&self, position: Position) -> bool {
        self.grid[position.y][position.x + 1].is_none()
    }

    fn is_left_position_empty(&self, position: Position) -> bool {
        self.grid[position.y][position.x - 1].is_none()
    }

    fn can_move_up(&self, position: Position, can_escape_flag: bool) -> bool {
        if self.exit.exit_direction == ExitSide::Top {
            self.exit.adjacent_grid.contains(&position)
        }

        !self.is_reach_the_top_bounds(position) && self.is_top_position_empty(position)
    }

    fn can_move_down(&self, position: Position, can_escape_flag: bool) -> bool {
        if self.exit.exit_direction == ExitSide::Bottom {
            self.exit.adjacent_grid.contains(&position)
        }

        !self.is_reach_the_bottom_bounds(position) && self.is_bottom_position_empty(position)
    }

    fn can_move_left(&self, position: Position, can_escape_flag: bool) -> bool {
        if self.exit.exit_direction == ExitSide::Left {
            self.exit.adjacent_grid.contains(&position)
        }

        !self.is_reach_the_left_bounds(position) && self.is_left_position_empty(position)
    }

    fn can_right_right(&self, position: Position, can_escape_flag: bool) -> bool {
        if self.exit.exit_direction == ExitSide::Right {
            self.exit.adjacent_grid.contains(&position)
        }

        !self.is_reach_the_right_bounds(position) && self.is_right_position_empty(position)
    }


    pub fn initialize(&mut self) {
        // load blocks to board
        self.board_with_blocks.blocks.iter().for_each(|block|{
            let block_in_game = BlockInGame{
                    block_id: block.block_id,
                    block_english_name: block.block_english_name.to_string(),
                    block_japanese_name: block.block_japanese_name.to_string(),
                    width: block.width,
                    height: block.height,
                    current_location: Position { x: block.initial_location.0 as isize, y: block.initial_location.1 as isize },
                    can_move_up: false,
                    can_move_down: false,
                    can_move_left: false,
                    can_move_right: false,
                    can_escape: block.can_escape,
            };
            self.blocks_in_game.push(block_in_game)
        });

        // initialize grid with empty
        let grid = vec![
            vec![None; self.board_with_blocks.width as usize];
            self.board_with_blocks.height as usize
        ]; // 初始化为空

        // initialize exit
        self.exit.exit_direction = *self.board_with_blocks.exit_position.side;
        if self.exit.exit_direction == ExitSide::Top {
            for i in 0..self.board_with_blocks.exit_position.length {
                self.exit.adjacent_grid.push(
                    Position{
                        x: (0 + self.board_with_blocks.exit_position.distance_to_edge + i) as isize,
                        y: (self.board_with_blocks.height - 1) as isize }
                )
            }
        } else if self.exit.exit_direction == ExitSide::Bottom {
            for i in 0..self.board_with_blocks.exit_position.length {
                self.exit.adjacent_grid.push(
                    Position{
                        x: (0 + self.board_with_blocks.exit_position.distance_to_edge + i) as isize,
                        y: 0 }
                )
            }
        } else if self.exit.exit_direction == ExitSide::Right {
            for i in 0..self.board_with_blocks.exit_position.length {
                self.exit.adjacent_grid.push(
                    Position{
                        x: (self.board_with_blocks.width - 1) as isize,
                        y: (0 + self.board_with_blocks.exit_position.distance_to_edge + i) as isize }
                )
            }
            
        } else if self.exit.exit_direction == ExitSide::Left {
            for i in 0..self.board_with_blocks.exit_position.length {
                self.exit.adjacent_grid.push(
                    Position{
                        x: 0,
                        y: (0 + self.board_with_blocks.exit_position.distance_to_edge + i) as isize }
                )
            }
        }


        // locate blocks to board
        self.blocks_in_game.iter().for_each(|block|{
            for i in 0..block.width {
                for j in 0..block.height {
                    grid[block.current_location.x + i][block.current_location.y + j] = Some(block);
                }
            }
        });


        self.blocks_in_game.iter().for_each(|block|{

            let can_escape_flag: bool = block.can_escape;

            // todo 思考一下这里怎么写

            let mut can_move_up_flag:bool = true;

            for i in 0..block.width {
                let position = Position{ x: block.current_location.x + i, y: block.current_location.y };
                if !self.can_move_up(position, can_escape_flag) {
                    can_move_up_flag = false;
                    break
                }
            }
        })






    }

}