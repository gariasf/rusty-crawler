use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

use crate::components::{Player, Position, Renderable};
use crate::map::{draw_map, new_map, TileType};
use crate::player::player_input;

mod components;
mod map;
mod player;

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();
        player_input(self, ctx);

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
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

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Rusty Crawler")
        .build()?;

    let mut gamestate = State { ecs: World::new() };
    gamestate.ecs.register::<Position>();
    gamestate.ecs.register::<Renderable>();
    gamestate.ecs.register::<Player>();

    gamestate.ecs.insert(new_map());

    gamestate
        .ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gamestate)
}
