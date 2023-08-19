#![allow(clippy::unnecessary_wraps)]
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
const PARTICLE_RADIUS: f32 = 12.0;
const COLORS: [Color; 7] = [
    Color::BLACK,
    Color::BLUE,
    Color::CYAN,
    Color::GREEN,
    Color::RED,
    Color::WHITE,
    Color::YELLOW,
];

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
            particle.velocity.y += GRAVITY / GRAVITY - 0.1;

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
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::from([0.1, 0.2, 0.3, 1.0]));

        for particle in &self.particles {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                particle.position,
                particle.radius,
                0.1,
                particle.color,
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

    let cb = ggez::ContextBuilder::new("particles_with_gravity", "ggez").window_mode(window_config);
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
