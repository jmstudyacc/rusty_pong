// in pong-0/src/main.rs

use ggez::conf::{Conf, WindowSetup};
use ggez::event::{run, EventHandler};
use ggez::graphics::set_window_title;
use ggez::timer::{delta, time_since_start};
use ggez::*;

// State is all of the data and information required to represent our game's current state
struct State {
    // enable text in the game
    text: graphics::Text,
    // tracking number of frames
    frames: usize,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        //let font = graphics::Font::default();
        let text = graphics::Text::new(("Hello World!", graphics::Font::default(), 48.0));
        let s = State { text, frames: 0 };
        Ok(s)
    }
}

// implements minimum of 2 callbacks update & draw!
impl EventHandler<GameError> for State {
    // GameResult is a utility provided by ggez to signify if there was an error or not
    // Internally it's just a Result<(), GameError>,
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        // When we want to update State we include that logic here
        Ok(())
    }

    // Context is how ggez gives you access to hardware
    // e.g. mouse, keyboard, timers, graphics, sound...
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // Drawables are drawn from their top-left corner.
        let offset = self.frames as f32 / 10.0;

        // offsets the text each frame
        let dest_point = glam::Vec2::new(offset, offset);

        // draws the value of text to the screen at dest_point
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        // add 1 to the frames counter on each callback
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }
}

// main() needs to return a Result as the game may load or fail
fn main() -> GameResult {
    // creating a new Conf file
    let _c = conf::Conf::new();

    // create a context builder using the context builder
    let cb = ggez::ContextBuilder::new("Hello World", "James")
        // edit the window_setup to change the window title
        .window_setup(conf::WindowSetup::default().title("Hello World"))
        // edit the window_mode to make the window resizable
        .window_mode(conf::WindowMode::default().resizable(true).maximized(false));

    // build the context and event loop using cb.build()?
    let (mut ctx, event_loop) = cb.build()?;

    // makes the window resizable
    ggez::graphics::set_resizable(&mut ctx, true)?;

    // change the window title after init
    ggez::graphics::set_window_title(&ctx, "Hello World!");

    // create a new state
    let state = State::new(&mut ctx)?;

    // run the game
    event::run(ctx, event_loop, state)
}
