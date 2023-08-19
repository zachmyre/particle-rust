#![allow(clippy::unnecessary_wraps)]
#[macro_use]
extern crate lazy_static;
extern crate rand;

use ggez::{
    conf::WindowSetup,
    event,
    glam::*,
    graphics::{self, Color, Drawable},
    winit::window,
    Context, GameResult,
};
use rand::{random, Rng};

const MAX_PARTICLES: i32 = 500;
const PARTICLE_RADIUS: f32 = 5.0;
// using lazy_static to generate colors (rust compiler fix)
lazy_static! {
    static ref NEON_GREEN: Color = Color::from([0.224, 1.0, 0.078, 1.0]);
    static ref NEON_BLUE: Color = Color::from([0.098, 0.454, 0.827, 1.0]);
    static ref NEON_PINK: Color = Color::from([1.0, 0.192, 0.333, 1.0]);
    static ref NEON_YELLOW: Color = Color::from([1.0, 0.835, 0.0, 1.0]);
    static ref NEON_CYAN: Color = Color::from([0.0, 0.659, 0.918, 1.0]);
    static ref NEON_ORANGE: Color = Color::from([1.0, 0.341, 0.133, 1.0]);
    static ref NEON_PURPLE: Color = Color::from([0.502, 0.0, 0.502, 1.0]);
}

lazy_static! {
    static ref COLORS: [Color; 7] = [
        *NEON_GREEN,
        *NEON_BLUE,
        *NEON_PINK,
        *NEON_YELLOW,
        *NEON_CYAN,
        *NEON_ORANGE,
        *NEON_PURPLE,
    ];
}

const GRAVITY: f32 = 0.9;
const ENERGY_LOSS: f32 = 0.8;

const SCREEN_HEIGHT: f32 = 768.0;
const SCREEN_WIDTH: f32 = 1024.0;

struct Particle {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    color: Color,
}

impl Particle {
    fn new(position: Vec2) -> Self {
        Particle {
            position,
            velocity: Vec2::new(0.0, 0.0),
            radius: PARTICLE_RADIUS,
            color: Self::pick_random_color(),
        }
    }

    fn pick_random_color() -> Color {
        let mut rng = rand::thread_rng();
        COLORS[rng.gen_range(0..COLORS.len())]
    }
}

struct MainState {
    particles: Vec<Particle>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut particles = Vec::new();
        for _ in 0..MAX_PARTICLES {
            particles.push(Particle::new(Vec2::new(
                rand::random::<f32>() * SCREEN_WIDTH,
                rand::random::<f32>() * SCREEN_HEIGHT,
            )));
        }

        Ok(MainState { particles })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for particle in &mut self.particles {
            // Apply gravity to the particle's velocity
            particle.velocity.y += GRAVITY;

            // particle.velocity.x += GRAVITY / GRAVITY - 0.1;

            // Update the particle's position based on its velocity
            particle.position += particle.velocity;

            // Check for collision with the floor
            if particle.position.y + particle.radius >= SCREEN_HEIGHT {
                // Set the particle's position to just touch the floor
                particle.position.y = SCREEN_HEIGHT - particle.radius;

                // Reverse the y-velocity (bounce) and reduce it to simulate energy loss
                particle.velocity.y = -particle.velocity.y * ENERGY_LOSS;
            }

            // Check for collision with the wall
            // if particle.position.x + particle.radius >= SCREEN_WIDTH {
            //     // Set the particle's position to just touch the wall
            //     particle.position.x = SCREEN_WIDTH - particle.radius;

            //     // Reverse the y-velocity (bounce) and reduce it to simulate energy loss
            //     particle.velocity.x = -particle.velocity.x * ENERGY_LOSS;
            // }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Create a new canvas to draw to.
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        for particle in &self.particles {
            // Draw the "glow"
            let glow_circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                particle.position,
                particle.radius * 1.2, // Larger than the actual particle
                0.1,
                Color::new(particle.color.r, particle.color.g, particle.color.b, 0.4), // Lower alpha for the glow
            )?;
            glow_circle.draw(&mut canvas, graphics::DrawParam::default());

            // Draw the actual particle
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                particle.position,
                particle.radius,
                0.1,
                particle.color, // Full alpha for the particle itself
            )?;
            circle.draw(&mut canvas, graphics::DrawParam::default());
        }

        // Draw the canvas to the main screen.
        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let window_config = ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT);
    let window_setup = ggez::conf::WindowSetup::default().title("Zach's Particle System");
    let cb = ggez::ContextBuilder::new("particles_with_gravity", "ggez")
        .window_mode(window_config)
        .window_setup(window_setup);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
