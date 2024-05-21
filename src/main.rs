use rltk::{Rltk, GameState};

struct State{}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "rusty crawler!");
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
    .with_title("Rusty Crawler")
    .build()?;

    let game_state = State{};
    rltk::main_loop(context, game_state)
}