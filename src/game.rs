use ggez::{conf::WindowMode, mint::{Point2, Vector2}, *};

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const SCREEN_WIDTH_MID: f32 = SCREEN_WIDTH / 2.0;
const SCREEN_HEIGHT_MID: f32 = SCREEN_HEIGHT / 2.0;

const BALL_VELOCITY: f32 = 500.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_RADIUS_MID: f32 = BALL_RADIUS / 2.0;

struct State {
    ball_pos: mint::Point2<f32>,
    ball_vel: mint::Vector2<f32>,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        State {
            ball_pos: mint::Point2{x: SCREEN_WIDTH_MID - BALL_RADIUS_MID, y: SCREEN_HEIGHT_MID - BALL_RADIUS_MID} ,
            ball_vel: mint::Vector2{x: BALL_VELOCITY, y: BALL_VELOCITY},
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ctx.time.delta().as_secs_f32();

        // ball movement
        self.ball_pos = mint::Point2 {
            x: self.ball_pos.x + self.ball_vel.x * delta_time,
            y: self.ball_pos.y + self.ball_vel.y * delta_time
        };

        // ball collisions
        if (self.ball_pos.x > SCREEN_WIDTH || self.ball_pos.x < 0.0) {
            self.ball_vel.x *= -1.0;
        }
        if (self.ball_pos.y > SCREEN_HEIGHT || self.ball_pos.y < 0.0) {
            self.ball_vel.y *= -1.0;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // create a canvas first to draw on
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        // make the ball mesh
        let ball = graphics::Mesh::new_circle(
            ctx, // sagt ggez wohin er malen soll - hier also auf unseren context
            graphics::DrawMode::fill(), // mal modus für unser ball
            self.ball_pos,
            BALL_RADIUS,
            0.1, // tolerance (idk) 
            graphics::Color::WHITE,
        )?;

        // now draw the ball onto canvas
        let mut draw_parameters = graphics::DrawParam::default();

        draw_parameters.dest(self.ball_pos);

        canvas.draw(&ball, draw_parameters);
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
