use crate::board::{Board, ExitSide};
use std::collections::HashMap;
use std::option::Option;
use ggez::{
    conf, event,
    input::keyboard::KeyCode,
    Context, GameResult,
};

use hecs::World;
use regex::Regex;
use std::path;
use crate::entity::*;
use crate::rendering::*;
use crate::components::*;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH, EXIT_KEY, EXIT_POSITIONS};

pub enum GameState {
    Running,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

pub struct Exit{
    adjacent_grid: Vec<Position>,
    exit_direction: ExitSide,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockInGame {
    pub block_id : u8,
    pub block_english_name: String,   // 英文名称
    pub block_japanese_name: String, // 日文名称
    pub width: u8,                   // 宽度
    pub height: u8,                  // 高度
    pub current_location: Position, // 当前位置(x,y)
    pub can_escape: bool,            // 是否可以逃脱
}

pub struct Game {
    pub board_with_blocks: Board,
    pub blocks_in_game : Vec<BlockInGame>,
    pub grid: Vec<Vec<Option<BlockInGame>>>,
    pub exit: Exit,
    pub world: World
}

// 检查棋子是否进入出口格子
// pub fn check_game_over(world: &World, game_state: &mut GameState) {
//     // 如果游戏已经结束，不再检查
//     if let GameState::GameOver = game_state {
//         return;
//     }
//
//     // 查找出口格子的位置
//     let exit_positions: Vec<PositionDuringGame> = world
//         .query::<(&PositionDuringGame, &Exit)>()
//         .iter()
//         .map(|(_, (pos, _))| *pos)
//         .collect();
//
//     // 查找 ID 为 0 的棋子的占用格子
//     if let Some((_, collision_volume)) = world
//         .query::<(&BlockId, &CollisionVolume)>()
//         .iter()
//         .find(|(_, (block_id, _))| block_id.block_id == "0")
//     {
//         // 判断是否有任意占用格子在出口格子列表中
//         if collision_volume.1
//             .occupied_cells
//             .iter()
//             .any(|cell| exit_positions.contains(cell))
//         {
//             *game_state = GameState::GameOver; // 游戏结束
//             println!("Game Over! Block ID 0 reached the exit.");
//         }
//     }
// }

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        unsafe {
            select_block( context);
            move_block(&mut self.world, context);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            run_rendering(&self.world, context);
        }

        Ok(())
    }
}

pub unsafe fn initialize_level(board_with_blocks: &Board, world: &mut World, exit_side: &ExitSide) {

    let mut map = vec!
    [
        vec!['.'; board_with_blocks.width as usize + 2];
        board_with_blocks.height as usize + 2
    ];

    MAP_WIDTH = board_with_blocks.width + 2;
    MAP_HEIGHT = board_with_blocks.height + 2;

    let map_len = map.len();

    for x in 0..map[0].len() {
        map[0][x] = 'W'; // 顶部
        map[map_len - 1][x] = 'W'; // 底部
    }

    let map_0_len = map[0].len();

// 填充左侧和右侧的墙壁
    for y in 0..map.len() {
        map[y][0] = 'W'; // 左侧
        map[y][map_0_len - 1] = 'W'; // 右侧
    }

    // 处理出口
    match board_with_blocks.exit_position.side {
        ExitSide::Bottom => {
            let start = board_with_blocks.exit_position.distance_to_edge as usize;
            let length = board_with_blocks.exit_position.length as usize;
            for i in start..(start + length).min(board_with_blocks.width as usize) {
                map[0][i + 1] = 'E';
            }
        }
        ExitSide::Top => {
            let start = board_with_blocks.exit_position.distance_to_edge as usize;
            let length = board_with_blocks.exit_position.length as usize;
            for i in start..(start + length).min(board_with_blocks.width as usize) {
                map[(board_with_blocks.height - 1 + 2) as usize][ i + 1 ] = 'E';
            }
        }
        ExitSide::Left => {
            let start = board_with_blocks.exit_position.distance_to_edge as usize;
            let length = board_with_blocks.exit_position.length as usize;
            for i in start..(start + length).min(board_with_blocks.height as usize) {
                map[ map_len - i - 1 ][0] = 'E';
            }
        }
        ExitSide::Right => {
            let start = board_with_blocks.exit_position.distance_to_edge as usize;
            let length = board_with_blocks.exit_position.length as usize;
            for i in start..(start + length).min(board_with_blocks.height as usize) {
                map[ map_len - i - 1 ][(board_with_blocks.width - 1 + 2 ) as usize] = 'E';
            }
        }
    }

    // 放置每个 Block 的初始位置
    for block in &board_with_blocks.blocks {
        let (x, y) = block.initial_location;
        map[ y as usize+1 ][x as usize+1] =
            block.block_id.to_string().chars().next().unwrap_or('?');
    }

    // 将二维数组转换为字符串
    let map_string = map
        .iter()
        .rev() // 反转行顺序
        .map(|row| row.iter().map(|&c| c.to_string()).collect::<Vec<_>>().join(" ")) // 每行字符用空格分隔
        .collect::<Vec<_>>()
        .join("\n"); // 行间用换行符分隔

    println!("{}", map_string);

    // 生成 Block 的宽高列表
    let block_info: Vec<(u8, u8, u8, bool)> = board_with_blocks
        .blocks
        .iter()
        .map(|block| (block.block_id.clone(), block.width, block.height, block.can_escape))
        .collect();

    // 转换为 HashMap
    let block_dict: HashMap<String, (u8, u8, bool)> = block_info
        .into_iter()
        .map(|(block_id, width, height, can_escape)| (block_id.to_string(), (width, height, can_escape)))
        .collect();

    // 使用调试模式打印
    println!("{:?}", block_dict);
    
    load_map(world, map_string, block_dict, *exit_side);
}

pub unsafe fn load_map(world: &mut World, map_string: String, block_dict: HashMap<String, (u8, u8, bool)>, exit_side: ExitSide ) {
    // read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    let digit_regex = Regex::new(r"^[0-9]$").unwrap();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = PositionDuringGame {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => {
                    create_floor(world, position);
                }
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "E" => {
                    unsafe {
                        EXIT_KEY = match exit_side {
                            ExitSide::Bottom => KeyCode::Down,
                            ExitSide::Top => KeyCode::Up,
                            ExitSide::Left => KeyCode::Left,
                            ExitSide::Right => KeyCode::Right,                            
                        };
                        let exit_position = PositionDuringGame {
                            x: position.x,
                            y: position.y,
                            z: 6 as u8
                        };
                        EXIT_POSITIONS.push(exit_position);
                    }
                    create_exit(world, position);
                }
                c if digit_regex.is_match(c) => {
                    let info = block_dict.get(c);
                    create_floor(world, position);
                    create_block(world, position, c, info);
                }
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

impl Game {
    pub fn new(board_with_blocks: Board) -> Self {
        Game {
            board_with_blocks,
            blocks_in_game: vec![],
            grid: vec![],
            exit: Exit { adjacent_grid: vec![], exit_direction: ExitSide::Bottom },
            world: Default::default()
        }
    }

    pub fn display(&self) {
        println!("--- Game State ---");
        println!("Board Dimensions: Width = {}, Height = {}",
                 self.board_with_blocks.width,
                 self.board_with_blocks.height
        );

        println!("\nExit Information:");
        println!("Exit Direction: {:?}", self.exit.exit_direction);
        println!("Adjacent Grids to Exit:");
        for position in &self.exit.adjacent_grid {
            println!("  - Position: ({}, {})", position.x, position.y);
        }

        println!("\nBlocks In Game:");
        for block in &self.blocks_in_game {
            println!(
                "Block ID: {}, English Name: {}, Japanese Name: {}, Position: ({}, {}), Width: {}, Height: {}, Can Escape: {}",
                block.block_id,
                block.block_english_name,
                block.block_japanese_name,
                block.current_location.x,
                block.current_location.y,
                block.width,
                block.height,
                block.can_escape,
            );

        }

        println!("\nGrid State:");
        // 从最后一行开始向上迭代
        for (y, row) in self.grid.iter().enumerate().rev() {
            for (_, cell) in row.iter().enumerate() {
                match cell {
                    Some(block) => print!("B{} ", block.block_id),
                    None => print!(".  "),
                }
            }
            println!("  (Row {})", y); // 保持 Row 的索引对应视觉上的行号
        }

        println!("--- End of Game State ---");
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

            if block.initial_location.0 > self.board_with_blocks.width {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the right border;\n", block.block_english_name)
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

            if block.initial_location.0 + block.width -1 > self.board_with_blocks.width {
                authorization_passed_flag = false;
                return_message.push_str(
                    &format!("Block name: {} initial location exceeds the right border;\n", block.block_english_name)
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


    pub unsafe fn initialize(&mut self) {
        // load blocks to board
        self.board_with_blocks.blocks.iter().for_each(|block|{
            let block_in_game = BlockInGame{
                    block_id: block.block_id,
                    block_english_name: block.block_english_name.to_string(),
                    block_japanese_name: block.block_japanese_name.to_string(),
                    width: block.width,
                    height: block.height,
                    current_location: Position { x: block.initial_location.0 as isize, y: block.initial_location.1 as isize },
                    can_escape: block.can_escape,
            };
            self.blocks_in_game.push(block_in_game)
        });

        // initialize grid with empty
        self.grid = vec![
            vec![None; self.board_with_blocks.width as usize];
            self.board_with_blocks.height as usize
        ]; // 初始化为空

        // initialize exit
        self.exit.exit_direction = self.board_with_blocks.exit_position.side;
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
                    let x = ( block.current_location.x + i as isize ) as usize;
                    let y = ( block.current_location.y - j as isize ) as usize;

                    self.grid[y][x] = Some(block.clone());
                }
            }
        });
    
        // load world to get ready for rendering
        let mut world = World::new();
        initialize_level(&self.board_with_blocks, &mut world, &self.board_with_blocks.exit_position.side);

        self.world = world;
    }

    pub fn start(self) -> GameResult {

        let context_builder = ggez::ContextBuilder::new("klotski_okehazama", "Yutong Du")
            .window_setup(conf::WindowSetup::default().title("Klotski Okehazama!"))
            .window_mode(conf::WindowMode::default().dimensions(800.0, 800.0))
            .add_resource_path(path::PathBuf::from("./resources"));

        let (context, event_loop) = context_builder.build()?;

        event::run(context, event_loop, self);

    }

}