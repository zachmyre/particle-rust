import * as rand from 'rand'; // Pretend 'rand' package exists.
import * as ggez from 'ggez'; // Imaginary import for the example.

const GRAVITY = 0.9;
const ENERGY_LOSS = 0.8;
const SCREEN_HEIGHT = 768.0;
const SCREEN_WIDTH = 1024.0;

interface Vector2 {
    x: number;
    y: number;
}

class Particle {
    position: Vector2;
    velocity: Vector2;
    radius: number;
    color: ggez.graphics.Color;

    constructor(position: Vector2) {
        this.position = position;
        this.velocity = { x: 0.0, y: 0.0 };
        this.radius = 5.0;
        this.color = ggez.graphics.Color.WHITE;
    }
}

class MainState {
    particles: Particle[];

    constructor(ctx: any) {
        this.particles = [];
        for (let i = 0; i < 100; i++) {
            this.particles.push(new Particle({
                x: rand.random() * 800.0,
                y: rand.random() * 600.0,
            }));
        }
    }

    update(ctx: any): void {
        for (let particle of this.particles) {
            particle.velocity.y += GRAVITY;
            particle.position.x += particle.velocity.x;
            particle.position.y += particle.velocity.y;

            if (particle.position.y + particle.radius >= SCREEN_HEIGHT) {
                particle.position.y = SCREEN_HEIGHT - particle.radius;
                particle.velocity.y = -particle.velocity.y * ENERGY_LOSS;
            }
        }
    }

    draw(ctx: any): void {
        const canvas = ggez.graphics.Canvas.fromFrame(ctx, ggez.graphics.Color.from([0.1, 0.2, 0.3, 1.0]));

        for (let particle of this.particles) {
            const circle = ggez.graphics.Mesh.newCircle(
                ctx,
                ggez.graphics.DrawMode.fill(),
                particle.position,
                particle.radius,
                0.1,
                particle.color
            );
            circle.draw(canvas, ggez.graphics.DrawParam.default());
        }

        canvas.finish(ctx);
    }
}

function main(): void {
    const windowConfig = ggez.conf.WindowMode.default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT);
    const cb = ggez.ContextBuilder.new("particles_with_gravity", "ggez").windowMode(windowConfig);
    const ctx = cb.build();
    const state = new MainState(ctx);
    ggez.event.run(ctx, state);
}

main();
