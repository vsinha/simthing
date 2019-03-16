use cgmath::*;
use ggez::conf::*;
use ggez::event::{self, EventHandler, EventsLoop, KeyCode, KeyMods};
use ggez::{Context, ContextBuilder, GameResult};

fn window_setup() -> (Context, EventsLoop) {
    ContextBuilder::new("simthing", "Ty Overby | Viraj Sinha")
        .window_mode(WindowMode::default().dimensions(500.0, 500.0))
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

struct Agent {
    position: Point2<f32>,
    velocity: Vector2<f32>,
}

struct MyGame {
    circle_mesh: ggez::graphics::Mesh,
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
            Color::from_rgb(255, 0, 0),
        )?;
        let agents = (0..10).map(|i| {
            let position = Point2::new(100.0, 100.0);
            let i = ((i as f32) / 10.0) * 3.14;
            let vx = f32::sin(i as f32);
            let vy = f32::cos(i as f32);
            let velocity = Vector2::new(vx, vy) / 10.0;
            Agent { position, velocity }
        });
        Ok(MyGame {
            circle_mesh,
            camera_pos: Vector3::new(0.0, 0.0, 0.0),
            agents: agents.collect(),
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for agent in &mut self.agents {
            agent.position += agent.velocity;
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
        use ggez::graphics::{clear, draw, present, DrawParam};
        clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let transform = Matrix4::from_translation(self.camera_pos);

        ggez::graphics::push_transform(ctx, Some(transform));
        ggez::graphics::apply_transformations(ctx)?;

        for agent in &self.agents {
            draw(
                ctx,
                &self.circle_mesh,
                DrawParam::default().dest(agent.position),
            )?;
        }
        ggez::timer::fps(ctx);
        present(ctx)?;
        ggez::graphics::pop_transform(ctx);
        ggez::timer::yield_now();
        Ok(())
    }
}
