use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub block_id : u8,
    pub block_english_name: String,   // 英文名称
    pub block_japanese_name: String, // 日文名称
    pub width: u8,                   // 宽度，假定为非负整数
    pub height: u8,                  // 高度，假定为非负整数
    pub initial_location: (u8, u8), // 初始位置(x,y)
    pub can_escape: bool,            // 是否可以逃脱
}

// 默认构造函数（可选）
impl Block {
    pub fn new(
        block_id:u8,
        block_english_name: &str,
        block_japanese_name: &str,
        width: u8,
        height: u8,
        initial_location: (u8, u8),
        can_escape: bool,
    ) -> Block {
        Block {
            block_id,
            block_english_name: block_english_name.to_string(),
            block_japanese_name: block_japanese_name.to_string(),
            width,
            height,
            initial_location,
            can_escape,
        }
    }

    pub fn display(&self) {
        println!(
            "Block Details:\n\
             - Blocks ID: {}\n\
             - English Name: {}\n\
             - Japanese Name: {}\n\
             - Dimensions: {} x {}\n\
             - Initial Location: ({}, {})\n\
             - Can Escape: {}",
            self.block_id,
            self.block_english_name,
            self.block_japanese_name,
            self.width,
            self.height,
            self.initial_location.0,
            self.initial_location.1,
            if self.can_escape { "Yes" } else { "No" }
        );
    }
}