use rltk::{Rltk, RGB};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

// Get a unique 1D index for a set of 2D coordinates
pub fn xy_index(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    // Boundary Walls
    for x in 0..80 {
        map[xy_index(x, 0)] = TileType::Wall;
        map[xy_index(x, 49)] = TileType::Wall;
    }
    for y in 0..50 {
        map[xy_index(0, y)] = TileType::Wall;
        map[xy_index(79, y)] = TileType::Wall;
    }

    let mut random_generator = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = random_generator.roll_dice(1, 79);
        let y = random_generator.roll_dice(1, 49);
        let index = xy_index(x, y);
        // (40, 25) is the middle of the map, the player will be starting there
        if index != xy_index(40, 25) {
            map[index] = TileType::Wall;
        }
    }
    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0., 0., 0.),
                    rltk::to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
