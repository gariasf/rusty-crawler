use rltk::{console, Point};
use specs::prelude::*;

use crate::components::{Name, Position};
use crate::map::Map;

use super::{Monster, Viewshed};

pub struct MonsterAI {}
impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (viewshed, _monster, name, position) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            let distance = rltk::DistanceAlg::Pythagoras
                .distance2d(Point::new(position.x, position.y), *player_pos);
            if distance < 1.5 {
                // Attack goes here
                console::log(&format!("{} shouts insults", name.name));
                return;
            }
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", name.name));
                let path = rltk::a_star_search(
                    map.xy_index(position.x, position.y) as i32,
                    map.xy_index(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );
                if path.success && path.steps.len() > 1 {
                    position.x = path.steps[1] as i32 % map.width;
                    position.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
