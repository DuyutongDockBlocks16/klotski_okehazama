pub struct Block {
    pub block_id : u8,
    pub block_english_name: String,   // 英文名称
    pub block_japanese_name: String, // 日文名称
    pub width: u8,                   // 宽度，假定为非负整数
    pub height: u8,                  // 高度，假定为非负整数
    pub initial_location: (i16, i16), // 初始位置(x,y)
    pub current_location: (i16, i16), // 当前的位置(x,y)
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
        initial_location: (i16, i16),
        can_escape: bool,
    ) -> Block {
        Block {
            block_id,
            block_english_name: block_english_name.to_string(),
            block_japanese_name: block_japanese_name.to_string(),
            width,
            height,
            initial_location,
            current_location: initial_location, // 初始时，当前位置等于初始位置
            can_escape,
        }
    }

    // 其他方法也可以写在这里
    pub fn display(&self) {
        println!(
            "Block Details:\n\
             - Blocks ID: {}\n\
             - English Name: {}\n\
             - Japanese Name: {}\n\
             - Dimensions: {} x {}\n\
             - Initial Location: ({}, {})\n\
             - Current Location: ({}, {})\n\
             - Can Escape: {}",
            self.block_id,
            self.block_english_name,
            self.block_japanese_name,
            self.width,
            self.height,
            self.initial_location.0,
            self.initial_location.1,
            self.current_location.0,
            self.current_location.1,
            if self.can_escape { "Yes" } else { "No" }
        );
    }

    // pub fn update_location(&mut self, new_location: (i16, i16)) {
    //     self.current_location = new_location;
    //     println!(
    //         "Location updated! New location is: ({}, {})",
    //         self.current_location.0, self.current_location.0 -1
    //     );
    // }

    // pub fn move_left(&mut self) {
    //     self.current_location = (self.current_location.0 - 1 , self.current_location.1 );
    //     println!(
    //         "Location move left! New location is: ({}, {})",
    //         self.current_location.0, self.current_location.1
    //     );
    // }
    //
    // pub fn move_right(&mut self) {
    //     self.current_location = (self.current_location.0 + 1 , self.current_location.1 );
    //     println!(
    //         "Location move right! New location is: ({}, {})",
    //         self.current_location.0, self.current_location.1
    //     );
    // }
    //
    // pub fn move_up(&mut self) {
    //     self.current_location = (self.current_location.0 , self.current_location.1 + 1 );
    //     println!(
    //         "Location move up! New location is: ({}, {})",
    //         self.current_location.0, self.current_location.1
    //     );
    // }
    //
    // pub fn move_down(&mut self) {
    //     self.current_location = (self.current_location.0 , self.current_location.1 - 1 );
    //     println!(
    //         "Location move down! New location is: ({}, {})",
    //         self.current_location.0, self.current_location.1
    //     );
    // }
    //
    // // 获取英文名称
    // pub fn get_block_english_name(&self) -> &str {
    //     &self.block_english_name
    // }
    //
    // // 获取日文名称
    // pub fn get_block_japanese_name(&self) -> &str {
    //     &self.block_japanese_name
    // }
    //
    // // 获取宽度
    // pub fn get_width(&self) -> u8 {
    //     self.width
    // }
    //
    // // 获取高度
    // pub fn get_height(&self) -> u8 {
    //     self.height
    // }
    //
    // // 获取初始位置
    // pub fn get_initial_location(&self) -> (i16, i16) {
    //     self.initial_location
    // }
    //
    // // 获取是否可以逃脱的状态
    // pub fn get_can_escape(&self) -> bool {
    //     self.can_escape
    // }
    //
    // // 获取当前位置的方法
    // pub fn get_current_location(&self) -> (i16, i16) {
    //     self.current_location
    // }
}