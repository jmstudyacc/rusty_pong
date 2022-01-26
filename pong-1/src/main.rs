// in pong-1/src/main.rs

use ggez::conf::{FullscreenType, NumSamples, WindowMode, WindowSetup};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::*;
use glam::Vec2;

// set constants to hold the screen size & half screen sizes
const WINDOW_WIDTH: f32 = 1280.0;
const HALF_WIDTH: f32 = WINDOW_WIDTH / 2.0;
const WINDOW_HEIGHT: f32 = 800.0;
const HALF_HEIGHT: f32 = WINDOW_HEIGHT / 2.0;
// constant holding array for BLACK color for graphics::clear(ctx: &mut Context, color: Color [f32; 4])
const BG_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

// create struct to store all of the data and information needed for the game
struct State {
    text: graphics::Text,
}

impl State {
    fn new(_ctx: &mut Context) -> GameResult<State> {
        // new state created with Hello Pong! created
        let text = graphics::Text::new(("Hello Pong!", graphics::Font::default(), 48.0));
        let s = State { text };
        Ok(s)
    }
}

impl EventHandler<GameError> for State {
    // nothing in the game needs updating at this stage
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        // clears the screen to the color specified - BLACK in this instance
        graphics::clear(ctx, BG_BLACK.into());

        // creating a Vec2 to store the centre point of the screen in relation to
        // the words 'Hello Pong!' with respect to font pixel size
        let dest_point = Vec2::new(
            HALF_WIDTH - self.text.width(ctx) / 2.0,
            HALF_HEIGHT - self.text.height(ctx),
        );
        // draws the value of text to the screen at dest_point
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => event::quit(ctx),
            _ => panic!(),
        }
    }
}

fn main() -> GameResult {
    // create a context builder
    let cb = ContextBuilder::new("Rusty Pong", "James Miles")
        // edit the window_setup to change the window title
        .window_setup(WindowSetup {
            title: "Hello Pong!".to_string(),
            samples: NumSamples::One,
            vsync: true,
            icon: "".to_string(),
            srgb: false,
        })
        .window_mode(WindowMode {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            min_height: 0.0,
            max_width: 0.0,
            max_height: 0.0,
            resizable: true,
            visible: true,
            resize_on_scale_factor_change: false,
        });

    // create a context & event loop
    let (mut ctx, event_loop) = cb.build()?;

    // set the filter mode to achieve a crisp 2D quality
    graphics::set_default_filter(&mut ctx, graphics::FilterMode::Nearest);

    // create a new state
    let state = State::new(&mut ctx)?;

    // run the game
    event::run(ctx, event_loop, state)
}
