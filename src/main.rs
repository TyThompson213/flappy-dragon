use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1,1, "Hello bracket temrinal");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Floppy Dragon")
        .build()?;

    main_loop(context,State{} )
}
