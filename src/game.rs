use ggez::{mint::{Point2, Vector2}, *};

const SCREEN_WIDTH: f32 = 400.0;
const SCREEN_HEIGHT: f32 = 400.0;
const BALL_VELOCITY: f32 = 200.0;

struct State {
    // delta_time: std::time::Duration,
    ball_pos: mint::Point2<f32>,
    ball_vel: mint::Vector2<f32>,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        State {
            // delta_time: std::time::Duration::new(0, 0),
            ball_pos: mint::Point2{x: 0.0, y: 0.0} ,
            ball_vel: mint::Vector2{x: BALL_VELOCITY, y: BALL_VELOCITY},
        }
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ctx.time.delta().as_secs_f32();
        self.ball_pos = mint::Point2 {
            x: self.ball_pos.x + self.ball_vel.x * delta_time,
            y: self.ball_pos.x + self.ball_vel.y * delta_time
        };
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
            19.0, // radius 
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
   // let state = State {
   //     delta_time: std::time::Duration::new(0, 0),
   //     ball_pos: na::Point2::new(screen_width, screen_height),
   //     ball_vel: na::Vector2::new(screen_width, screen_height),
   // };
    let context_builder = ggez::ContextBuilder::new("pingpangpong", "Lai");
    let (mut context, event_loop) = context_builder.build().unwrap();
    let mut state = State::new(&mut context);
    context.gfx.set_window_title("pingpangpong");
    // context.gfx.set_resizable(true);

    ggez::event::run(context, event_loop, state);
}
