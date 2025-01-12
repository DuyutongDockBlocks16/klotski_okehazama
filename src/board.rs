use crate::block::Block;

#[derive(Debug)]
pub enum ExitSide {
    Top,    // 上侧
    Bottom, // 下侧
    Left,   // 左侧
    Right,  // 右侧
}

#[derive(Debug)]
pub struct ExitPosition {
    pub side: ExitSide, // 出口方向
    pub distance_to_edge: u8, // 在左边和右边就是到上边缘的距离，在上侧和下侧就是到左边的距离
    pub length: u8,
}

pub struct Board {
    pub width: u8,                   // 宽度
    pub height: u8,                  // 高度
    pub exit_position: ExitPosition,//出口位置
    pub blocks: Vec<Block>
}

impl Board {
    // 创建新的 Box
    pub fn new(width: u8, height: u8, exit_position: ExitPosition) -> Self {
        Board {
            width,
            height,
            exit_position,
            blocks: vec![],
        }
    }

    // 显示 Box 的信息
    pub fn display(&self) {
        println!(
            "Board - Width: {}, Height: {}, Exit Side: {:?}, Distance to Edge: {}, Length: {}",
            self.width,
            self.height,
            self.exit_position.side,
            self.exit_position.distance_to_edge,
            self.exit_position.length
        );

        if self.blocks.is_empty() {
            println!("No blocks on the board.");
        } else {
            println!("Blocks on the board:");
            for (i, block) in self.blocks.iter().enumerate() {
                println!(
                    "  Block {}: Name (EN): {}, Name (JP): {}, Width: {}, Height: {}, Initial Location: {:?}, Current Location: {:?}, Can Escape: {}",
                    block.block_id,
                    block.block_english_name,
                    block.block_japanese_name,
                    block.width,
                    block.height,
                    block.initial_location,
                    block.current_location,
                    block.can_escape
                );
            }
        }
    }

    pub fn add_block(&mut self, new_block: Block){
        self.blocks.push(new_block);
    }
}