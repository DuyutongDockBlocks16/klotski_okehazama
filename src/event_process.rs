use crate::components::*;
use crate::events::*;
use ggez::Context;
use hecs::World;

use std::collections::HashMap;

pub fn run_process_events(world: &mut World, context: &mut Context) {
    let events = {
        let mut query = world.query::<&mut crate::components::EventQueue>();
        let events = query
            .iter()
            .next()
            .unwrap()
            .1
            .events
            .drain(..)
            .collect::<Vec<_>>();

        events
    };

    let mut new_events = Vec::new();

    let mut query = world.query::<(&PositionDuringGame, &ExitDuringGame)>();
    let box_spots_by_position: HashMap<(u8, u8), &ExitDuringGame> = query
        .iter()
        .map(|(_, t)| ((t.0.x, t.0.y), t.1))
        .collect::<HashMap<_, _>>();

    let mut query = world.query::<&mut AudioStore>();
    let audio_store = query.iter().next().unwrap().1;

    for event in events {
        println!("New event: {:?}", event);

        match event {
            Event::BlockHitObstacle => {
                // play sound here
                audio_store.play_sound(context, "wall");
            }
            Event::BlockMoved(BlockMoved { entity }) => {
                if let Ok(the_box) = world.get::<&BlockDuringGame>(entity) {
                    if let Ok(box_position) = world.get::<&PositionDuringGame>(entity) {
                        // Check if there is a spot on this position, and if there
                        // is if it's the correct or incorrect type
                        if let Some(box_spot) =
                            box_spots_by_position.get(&(box_position.x, box_position.y))
                        {
                            new_events.push(Event::TargetBlockReachExit);
                        }
                    }
                }
            }
            Event::TargetBlockReachExit => {
                // play sound here
                let sound = "win";

                audio_store.play_sound(context, sound);
            }
        }
    }

    // Finally add events back into the world
    {
        let mut query = world.query::<&mut EventQueue>();
        let event_queue = query.iter().next().unwrap().1;
        event_queue.events.append(&mut new_events);
    }
}