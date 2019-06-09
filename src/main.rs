use cgmath::*;
use ggez::conf::*;
use ggez::event::{self, EventHandler, EventsLoop, KeyCode, KeyMods};
use ggez::*;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;

mod ring;

static HEIGHT: f32 = 400.0;
static WIDTH: f32 = 600.0;

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

struct Agent {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    target: Vector2<f32>,
    trail: Vec<Vector2<f32>>,
}

struct MyGame {
    circle_mesh: ggez::graphics::Mesh,
    target_mesh: ggez::graphics::Mesh,
    trail_mesh: ggez::graphics::Mesh,
    camera_pos: Vector3<f32>,
    agents: Vec<Agent>,
}

fn random_vec2(w: f32, h: f32) -> Vector2<f32> {
    let mut rng = rand::thread_rng();
    Vector2::new(rng.gen_range(0., w), rng.gen_range(0., h))
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

        let agents = (0..10).map(|_i| {
            let position = random_vec2(WIDTH, HEIGHT);
            let velocity = random_vec2(WIDTH, HEIGHT);
            let target = random_vec2(WIDTH, HEIGHT);
            let trail = (0..10).map(|_i| position.clone()).collect();
            Agent {
                position,
                velocity,
                target,
                trail,
            }
        });

        Ok(MyGame {
            circle_mesh,
            target_mesh,
            trail_mesh,
            camera_pos: Vector3::new(0.0, 0.0, 0.0),
            agents: agents.collect(),
        })
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        for agent in &mut self.agents {
            // pick a new target location
            if rand::random() {
                agent.target = random_vec2(WIDTH, HEIGHT);
            }
            let new_direction = Vector2::normalize(agent.target - agent.position);
            agent.velocity = Vector2::normalize(agent.velocity + new_direction) * 1.0;
            let mut prev_position = agent.position;
            agent.position += agent.velocity;
            for position in agent.trail.iter_mut() {
                let temp = *position;
                *position = prev_position;
                prev_position = temp;
            }
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
