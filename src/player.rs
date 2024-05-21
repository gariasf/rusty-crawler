use std::cmp::{max, min};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs::{Join, World};

use crate::components::{Player, Position};
use crate::map::{xy_index, TileType};
use crate::State;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_index = xy_index(pos.x + delta_x, pos.y + delta_y);

        if map[destination_index] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gamestate: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gamestate.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gamestate.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gamestate.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gamestate.ecs),
            _ => {}
        },
    }
}