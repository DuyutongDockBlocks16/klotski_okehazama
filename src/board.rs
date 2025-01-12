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
    width: u8,                   // 宽度
    height: u8,                  // 高度
    exit_position: ExitPosition,//出口位置
    blocks: Vec<Block>
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
    }

    pub fn add_block(&mut self, new_block: Block){
        self.blocks.push(new_block);
    }
}