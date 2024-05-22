use rltk::{field_of_view, Point};
use specs::prelude::*;

use crate::components::Player;
use crate::map::Map;

use super::{Position, Viewshed};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (entity, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed.visible_tiles.retain(|point| {
                    point.x >= 0 && point.x < map.width && point.y >= 0 && point.y < map.height
                });

                // If this is the player, reveal what they can see
                let player: Option<&Player> = player.get(entity);
                if let Some(_player) = player {
                    for tile in map.visible_tiles.iter_mut() {
                        *tile = false
                    }

                    for visibility in viewshed.visible_tiles.iter() {
                        let index = map.xy_index(visibility.x, visibility.y);
                        map.visible_tiles[index] = true;
                        map.revealed_tiles[index] = true;
                    }
                }
            }
        }
    }
}
