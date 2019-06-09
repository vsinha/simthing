use cgmath::*;
use ggez::conf::*;
use ggez::event::{self, EventHandler, EventsLoop, KeyCode, KeyMods};
use ggez::*;
use ggez::{Context, ContextBuilder, GameResult};

mod agent;
use agent::Agent;

mod config;
use config::{HEIGHT, WIDTH};

mod ring;
mod vec;

fn window_setup() -> (Context, EventsLoop) {
    ContextBuilder::new("simthing", "Ty Overby | Viraj Sinha")
        .window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT))
        .window_setup(WindowSetup::default().title("simthing"))
        .build()
        .unwrap()
}

fn main() {
    let (mut ctx, mut event_loop) = window_setup();
    let mut my_game = MyGame::new(&mut ctx).unwrap();

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    circle_mesh: ggez::graphics::Mesh,
    target_mesh: ggez::graphics::Mesh,
    trail_mesh: ggez::graphics::Mesh,
    camera_pos: Vector3<f32>,
    agents: Vec<Agent>,
}

impl MyGame {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        use ggez::graphics::{Color, DrawMode, FillOptions, Mesh};
        let circle_mesh = Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            [0.0, 0.0],
            5.0,
            0.1,
            Color::from_rgb(200, 100, 0),
        )?;

        let trail_mesh = Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            [0.0, 0.0],
            2.0,
            0.1,
            Color::from_rgb(80, 80, 80),
        )?;

        let target_mesh = Mesh::new_circle(
            ctx,
            DrawMode::Fill(FillOptions::default()),
            [0.0, 0.0],
            1.0,
            0.1,
            Color::from_rgb(40, 40, 40),
        )?;

        let agents = (0..10).map(|_i| Agent::new()).collect();

        Ok(MyGame {
            circle_mesh,
            target_mesh,
            trail_mesh,
            camera_pos: Vector3::new(0.0, 0.0, 0.0),
            agents,
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for agent in &mut self.agents {
            agent.pick_random_target();
            agent.update();
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        let vector = match keycode {
            KeyCode::W => Vector3::new(0.0, 1.0, 0.0),
            KeyCode::S => Vector3::new(0.0, -1.0, 0.0),
            KeyCode::A => Vector3::new(1.0, 0.0, 0.0),
            KeyCode::D => Vector3::new(-1.0, 0.0, 0.0),
            _ => Vector3::new(0.0, 0.0, 0.0),
        };
        let vector = vector * 5.0;
        self.camera_pos += vector;
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 0.00].into());

        let transform = Matrix4::from_translation(self.camera_pos);

        graphics::push_transform(ctx, Some(transform));
        graphics::apply_transformations(ctx)?;

        for agent in &self.agents {
            graphics::draw(
                ctx,
                &self.circle_mesh,
                graphics::DrawParam::default().dest(Point2::from_vec(agent.position)),
            )?;

            // graphics::draw(
            //     ctx,
            //     &self.target_mesh,
            //     graphics::DrawParam::default().dest(Point2::from_vec(agent.target)),
            // )?;

            // for position in agent.trail.iter() {
            //     graphics::draw(
            //         ctx,
            //         &self.trail_mesh,
            //         graphics::DrawParam::default().dest(Point2::from_vec(*position)),
            //     )?;
            // }
        }

        println!("FPS: {}", timer::fps(ctx));

        graphics::present(ctx)?;
        graphics::pop_transform(ctx);
        timer::yield_now();
        Ok(())
    }
}
