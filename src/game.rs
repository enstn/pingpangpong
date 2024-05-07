use ggez::{conf::WindowMode, mint::{Point2, Vector2}, *};
use ggez::graphics::Rect;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH_MID: f32 = SCREEN_WIDTH / 2.0;
const SCREEN_HEIGHT_MID: f32 = SCREEN_HEIGHT / 2.0;

const BALL_VELOCITY: f32 = 500.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_RADIUS_MID: f32 = BALL_RADIUS / 2.0;

const PAD_VELOCITY: f32 = 500.0;
const PAD_LENGTH: f32 = 100.0;
const PAD_WIDTH: f32 = 10.0;
const PAD_LEFT_EDGE: f32 = 40.0;

struct Pad {
    rect: Rect,
    pos: mint::Point2<f32>,
    vel: mint::Vector2<f32>,
}

impl Pad {
    pub fn new() -> Self {
        Pad {
            rect: ggez::graphics::Rect::new(
                PAD_LEFT_EDGE,
                SCREEN_HEIGHT_MID - (PAD_LENGTH / 2.0),
                PAD_WIDTH,
                PAD_LENGTH,
            ),
            pos: mint::Point2{x: PAD_LEFT_EDGE, y: SCREEN_HEIGHT_MID},
            vel: mint::Vector2{x: 0.0, y: PAD_VELOCITY},
        }
    }
}

struct Ball {
    pos: mint::Point2<f32>,
    vel: mint::Vector2<f32>,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: mint::Point2{x: SCREEN_WIDTH_MID - BALL_RADIUS_MID, y: SCREEN_HEIGHT_MID - BALL_RADIUS_MID},
            vel: mint::Vector2{x: BALL_VELOCITY, y: BALL_VELOCITY},
        }
    }
}

struct State {
    ball: Ball,
    pad: Pad,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        State {
            ball: Ball::new(),
            pad: Pad::new(),
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ctx.time.delta().as_secs_f32();

        // ball movement
        self.ball.pos = mint::Point2 {
            x: self.ball.pos.x + self.ball.vel.x * delta_time,
            y: self.ball.pos.y + self.ball.vel.y * delta_time
        };

        // ball collisions
        if (self.ball.pos.x > SCREEN_WIDTH || self.ball.pos.x < 0.0) {
            self.ball.vel.x *= -1.0;
        }
        if (self.ball.pos.y > SCREEN_HEIGHT || self.ball.pos.y < 0.0) {
            self.ball.vel.y *= -1.0;
        }

        // TODO: pad_movement with key input
        // TODO: pad-ball collisions
        // TODO: Score & reset

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // create a canvas first to draw on
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // make the ball mesh
        let ball = graphics::Mesh::new_circle(
            ctx, // sagt ggez wohin er malen soll - hier also auf unseren context
            graphics::DrawMode::fill(), // mal modus für unser ball
            self.ball.pos,
            BALL_RADIUS,
            0.1, // tolerance (idk) 
            graphics::Color::WHITE,
        )?;

        // make the pad mesh
        let pad = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::stroke(1.0),
                self.pad.rect,
                graphics::Color::WHITE,
            )?;

        // TODO: Score System

        // set the params for drawing (this gets the position done)
        let mut draw_parameters = graphics::DrawParam::default();

        draw_parameters.dest(self.ball.pos);

        // I like it, picasso
        canvas.draw(&ball, draw_parameters);
        canvas.draw(&pad, draw_parameters);
        canvas.finish(ctx);
        Ok(())
    }
}

pub fn main() {

    let context_builder = ggez::ContextBuilder::new("pingpangpong", "Lai")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT).borderless(true));
    let (mut context, event_loop) = context_builder.build().unwrap();
    let mut state = State::new(&mut context);
    context.gfx.set_window_title("pingpangpong");
    // context.gfx.set_resizable(true);

    ggez::event::run(context, event_loop, state);
}
