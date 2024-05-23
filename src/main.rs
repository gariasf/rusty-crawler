use rltk::{GameState, Point, Rltk, RGB};
use specs::prelude::*;

use crate::components::{BlocksTile, Monster, Name, Player, Position, Renderable, Viewshed};
use crate::map::{draw_map, Map};
use crate::map_indexing_system::MapIndexingSystem;
use crate::monster_ai_system::MonsterAI;
use crate::player::player_input;
use crate::visibility_system::VisibilitySystem;

mod components;
mod map;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;
mod map_indexing_system;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Runinng,
}

struct State {
    ecs: World,
    runstate: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut visibility = VisibilitySystem {};
        visibility.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem{};
        mapindex.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        if self.runstate == RunState::Runinng {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        ctx.cls();
        draw_map(&self.ecs, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        for (pos, render) in (&positions, &renderables).join() {
            let index = map.xy_index(pos.x, pos.y);
            if map.visible_tiles[index] {
                ctx.set(
                    pos.x,
                    pos.y,
                    render.foreground,
                    render.background,
                    render.glyph,
                );
            }
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rusty Crawler")
        .build()?;

    let mut gamestate = State {
        ecs: World::new(),
        runstate: RunState::Runinng,
    };
    gamestate.ecs.register::<Position>();
    gamestate.ecs.register::<Renderable>();
    gamestate.ecs.register::<Player>();
    gamestate.ecs.register::<Viewshed>();
    gamestate.ecs.register::<Monster>();
    gamestate.ecs.register::<Name>();
    gamestate.ecs.register::<BlocksTile>();

    let map = Map::new_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = rltk::RandomNumberGenerator::new();
    for (index, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();

        let glyph: rltk::FontCharType;
        let name: String;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = rltk::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = rltk::to_cp437('o');
                name = "Orc".to_string();
            }
        }

        gamestate
            .ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                foreground: RGB::named(rltk::RED),
                background: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, index),
            })
            .with(BlocksTile{})
            .build();
    }

    gamestate.ecs.insert(map);
    gamestate.ecs.insert(Point::new(player_x, player_y));

    gamestate
        .ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .with(Name {
            name: "Player".to_string(),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    rltk::main_loop(context, gamestate)
}
