use hecs::Entity;

#[derive(Debug)]
pub struct BlockMoved {
    pub entities: Vec<Entity>,
}

#[derive(Debug)]
pub enum Event {
    // Fired when the player hits an obstacle like a wall
    BlockHitObstacle,

    // Fired when an entity is moved
    BlockMoved(BlockMoved),

    // Fired when the box is placed on a spot
    TargetBlockReachExit,
}