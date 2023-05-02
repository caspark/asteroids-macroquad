use macroquad::prelude::*;

struct Player {
    pos: Vec2,
    angle: f32,
    velocity: Vec2,
    shot_cooldown: f32,
}

struct Bullet {
    pos: Vec2,
    velocity: Vec2,
    life: f32,
}

struct Asteroid {
    pos: Vec2,
    velocity: Vec2,
    size: f32,
}

const PLAYER_SIZE: f32 = 10f32;

const MAX_SPEED: f32 = 1000.0f32;
const ACCELERATION: f32 = 500f32;
const FRICTION: f32 = 0.1f32;
const TURN_SPEED: f32 = 5f32;
const SHOT_COOLDOWN: f32 = 0.5f32;

const BULLET_MAX_LIFE: f32 = 1f32;
const BULLET_SPEED: f32 = 1000f32;
const BULLET_RADIUS: f32 = 5f32;

const ASTEROID_MAX_SPEED: f32 = 100f32;
const ASTEROID_COUNT: u32 = 10;

#[macroquad::main("BasicShapes")]
async fn main() {
    request_new_screen_size(1024f32, 768f32);

    next_frame().await; // wait for screen resize

    let screen = vec2(screen_width(), screen_height());

    let mut player = Player {
        pos: vec2(screen.x / 2.0, screen.y / 2.0),
        angle: 0f32,
        velocity: Vec2::ZERO,
        shot_cooldown: 0.0,
    };

    let mut asteroids = (0..ASTEROID_COUNT)
        .map(|_| Asteroid {
            pos: vec2(
                rand::gen_range(0.0, screen.x),
                rand::gen_range(0.0, screen.y),
            ),
            velocity: vec2(
                rand::gen_range(-ASTEROID_MAX_SPEED, ASTEROID_MAX_SPEED),
                rand::gen_range(-ASTEROID_MAX_SPEED, ASTEROID_MAX_SPEED),
            ),
            size: rand::gen_range(10.0, 50.0),
        })
        .collect::<Vec<_>>();

    let mut bullets = Vec::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            println!("Escape pressed, exiting...");
            break;
        }

        let mut draw_thrust = false;
        let thrust = if is_key_down(KeyCode::W) {
            draw_thrust = true;
            vec2(
                ACCELERATION * player.angle.cos(),
                ACCELERATION * player.angle.sin(),
            )
        } else {
            Vec2::ZERO
        };

        let turning = if is_key_down(KeyCode::A) {
            -1.0
        } else if is_key_down(KeyCode::D) {
            1.0
        } else {
            0.0 // no turning
        };

        let shooting = is_key_down(KeyCode::S) && player.shot_cooldown <= 0.0;

        player.angle += turning * TURN_SPEED * get_frame_time();
        player.velocity += thrust * get_frame_time();
        player.shot_cooldown -= get_frame_time();
        player.shot_cooldown = player.shot_cooldown.max(0.0);

        // friction
        let player_speed_mag = player.velocity.length();
        if player_speed_mag > 0.0 {
            player.velocity -= player.velocity.normalize() * FRICTION * get_frame_time();
        }

        // clamping
        if player_speed_mag > MAX_SPEED {
            player.velocity = player.velocity.normalize() * MAX_SPEED;
        } else if player_speed_mag < 0.0 {
            player.velocity = Vec2::ZERO;
        }

        player.pos += player.velocity * get_frame_time();

        // warping
        if player.pos.x > screen.x {
            player.pos.x = 0.0;
        } else if player.pos.x < 0.0 {
            player.pos.x = screen.x;
        }
        if player.pos.y > screen.y {
            player.pos.y = 0.0;
        } else if player.pos.y < 0.0 {
            player.pos.y = screen.y;
        }

        if shooting {
            player.shot_cooldown = SHOT_COOLDOWN;
            bullets.push(Bullet {
                pos: player.pos,
                velocity: vec2(
                    player.angle.cos() * BULLET_SPEED,
                    player.angle.sin() * BULLET_SPEED,
                ),
                life: BULLET_MAX_LIFE,
            });
        }

        for bullet in bullets.iter_mut() {
            bullet.life -= get_frame_time();
            bullet.pos += bullet.velocity * get_frame_time();

            // warping
            if bullet.pos.x > screen.x {
                bullet.pos.x = 0.0;
            } else if bullet.pos.x < 0.0 {
                bullet.pos.x = screen.x;
            }
            if bullet.pos.y > screen.y {
                bullet.pos.y = 0.0;
            } else if bullet.pos.y < 0.0 {
                bullet.pos.y = screen.y;
            }
        }
        bullets.retain(|bullet| bullet.life > 0.0);

        for asteroid in asteroids.iter_mut() {
            asteroid.pos += asteroid.velocity * get_frame_time();

            // warping
            if (asteroid.pos.x + asteroid.size) < 0.0 {
                asteroid.pos.x = screen.x + asteroid.size;
            } else if (asteroid.pos.x - asteroid.size) > screen.x {
                asteroid.pos.x = -asteroid.size;
            }
            if (asteroid.pos.y + asteroid.size) < 0.0 {
                asteroid.pos.y = screen.y + asteroid.size;
            } else if (asteroid.pos.y - asteroid.size) > screen.y {
                asteroid.pos.y = -asteroid.size;
            }
        }
        asteroids.retain(|asteroid| {
            let mut collision = false;
            bullets.retain(|bullet| {
                if (bullet.pos - asteroid.pos).length() < asteroid.size + BULLET_RADIUS {
                    collision = true;
                    false
                } else {
                    true
                }
            });

            if (player.pos - asteroid.pos).length() < asteroid.size + PLAYER_SIZE {
                collision = true;
            }

            !collision
        });

        clear_background(BLACK);

        for bullet in bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, BULLET_RADIUS, GREEN);
        }

        for asteroid in asteroids.iter() {
            draw_circle_lines(asteroid.pos.x, asteroid.pos.y, asteroid.size, 1.0, WHITE);
        }

        if draw_thrust {
            draw_circle(
                player.pos.x - player.angle.cos() * PLAYER_SIZE,
                player.pos.y - player.angle.sin() * PLAYER_SIZE,
                8f32,
                YELLOW,
            );
        }

        draw_triangle(
            vec2(
                player.pos.x + player.angle.cos() * PLAYER_SIZE,
                player.pos.y + player.angle.sin() * PLAYER_SIZE,
            ),
            vec2(
                player.pos.x
                    + (player.angle + std::f32::consts::PI / 4.0 + std::f32::consts::PI).cos()
                        * PLAYER_SIZE,
                player.pos.y
                    + (player.angle + std::f32::consts::PI / 4.0 + std::f32::consts::PI).sin()
                        * PLAYER_SIZE,
            ),
            vec2(
                player.pos.x
                    + (player.angle - std::f32::consts::PI / 4.0 + std::f32::consts::PI).cos()
                        * PLAYER_SIZE,
                player.pos.y
                    + (player.angle - std::f32::consts::PI / 4.0 + std::f32::consts::PI).sin()
                        * PLAYER_SIZE,
            ),
            WHITE,
        );

        draw_line(
            player.pos.x,
            player.pos.y,
            player.pos.x + player.velocity.x,
            player.pos.y + player.velocity.y,
            1.0,
            BLUE,
        );
        draw_line(
            player.pos.x,
            player.pos.y,
            player.pos.x + player.angle.cos() * 50f32,
            player.pos.y + player.angle.sin() * 50f32,
            1.0,
            RED,
        );
        draw_circle(player.pos.x, player.pos.y, 2f32, RED);

        next_frame().await
    }
}
