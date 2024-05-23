use std::cmp::{max, min};

use rltk::{console, Point, Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs::{Join, World};

use crate::components::{CombatStats, Player, Position, Viewshed};
use crate::map::Map;
use crate::{RunState, State};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_index = map.xy_index(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_index].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {}
                Some(t) => {
                    // Attack it
                    console::log(&format!("From Hell's Heart, I stab thee!"));
                    return; // So we don't move after attacking
                }
            }
        }
        if !map.blocked[destination_index] {
            viewshed.dirty = true;

            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            let mut player_pos = ecs.write_resource::<Point>();
            player_pos.x = pos.x;
            player_pos.y = pos.y;
        }
    }
}

pub fn player_input(gamestate: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => return RunState::Paused, // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gamestate.ecs)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gamestate.ecs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gamestate.ecs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gamestate.ecs)
            }

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => {
                try_move_player(1, -1, &mut gamestate.ecs)
            }

            VirtualKeyCode::Numpad7 | VirtualKeyCode::U => {
                try_move_player(-1, -1, &mut gamestate.ecs)
            }

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => {
                try_move_player(1, 1, &mut gamestate.ecs)
            }

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => {
                try_move_player(-1, 1, &mut gamestate.ecs)
            }

            _ => return RunState::Paused,
        },
    }

    RunState::Runinng
}
