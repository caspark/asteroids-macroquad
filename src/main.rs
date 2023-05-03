use macroquad::{
    audio::{load_sound, play_sound_once, PlaySoundParams},
    prelude::*,
};

const DEBUG_DRAW: bool = false;

struct Player {
    pos: Vec2,
    angle: f32,
    velocity: Vec2,
    shot_cooldown: f32,
}

impl Player {
    fn draw(&self, draw_thrust: bool) {
        if draw_thrust {
            draw_circle(
                self.pos.x - self.angle.cos() * PLAYER_SIZE,
                self.pos.y - self.angle.sin() * PLAYER_SIZE,
                8f32,
                YELLOW,
            );
        }

        draw_triangle(
            vec2(
                self.pos.x + self.angle.cos() * PLAYER_SIZE,
                self.pos.y + self.angle.sin() * PLAYER_SIZE,
            ),
            vec2(
                self.pos.x
                    + (self.angle + std::f32::consts::PI / 4.0 + std::f32::consts::PI).cos()
                        * PLAYER_SIZE,
                self.pos.y
                    + (self.angle + std::f32::consts::PI / 4.0 + std::f32::consts::PI).sin()
                        * PLAYER_SIZE,
            ),
            vec2(
                self.pos.x
                    + (self.angle - std::f32::consts::PI / 4.0 + std::f32::consts::PI).cos()
                        * PLAYER_SIZE,
                self.pos.y
                    + (self.angle - std::f32::consts::PI / 4.0 + std::f32::consts::PI).sin()
                        * PLAYER_SIZE,
            ),
            WHITE,
        );

        if DEBUG_DRAW {
            draw_line(
                self.pos.x,
                self.pos.y,
                self.pos.x + self.velocity.x,
                self.pos.y + self.velocity.y,
                1.0,
                BLUE,
            );
        }
    }
}

struct Bullet {
    pos: Vec2,
    velocity: Vec2,
    life: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct AsteroidPrototype {
    size: f32,
    speed: f32,
    score: u32,
}

struct Asteroid {
    pos: Vec2,
    velocity: Vec2,
    prototype_index: usize,
}

impl Asteroid {
    pub fn new(prototype_index: usize, pos: Vec2) -> Self {
        assert!(prototype_index < ASTEROID_CONFIG.len());

        let angle = rand::gen_range(0.0f32, ::std::f32::consts::PI * 2.0);
        let velocity = vec2(angle.cos(), angle.sin()) * ASTEROID_CONFIG[prototype_index].speed;

        Self {
            pos,
            velocity,
            prototype_index,
        }
    }

    pub fn new_inside(prototype_index: usize, bounds: Vec2) -> Self {
        let pos = vec2(
            rand::gen_range(0.0, bounds.x),
            rand::gen_range(0.0, bounds.y),
        );
        Self::new(prototype_index, pos)
    }

    pub fn spawn_many(
        v: &mut Vec<Asteroid>,
        count: u32,
        bounds: Vec2,
        avoid_pos: Vec2,
        avoid_dist: f32,
    ) {
        while v.len() < count as usize {
            let asteroid = Asteroid::new_inside(0, bounds);
            if asteroid.pos.distance(avoid_pos) < asteroid.prototype().size + avoid_dist {
                continue;
            }
            for other in v.iter() {
                if asteroid.pos.distance(other.pos)
                    < asteroid.prototype().size + other.prototype().size
                {
                    continue;
                }
            }

            // no collisions so we're good to go
            v.push(asteroid);
        }
    }

    fn prototype(&self) -> &AsteroidPrototype {
        &ASTEROID_CONFIG[self.prototype_index]
    }

    pub fn size(&self) -> f32 {
        self.prototype().size
    }

    pub fn score(&self) -> u32 {
        self.prototype().score
    }

    pub fn split(&self) -> Option<[Asteroid; 2]> {
        if self.prototype_index == ASTEROID_CONFIG.len() - 1 {
            None
        } else {
            Some([
                Asteroid::new(self.prototype_index + 1, self.pos),
                Asteroid::new(self.prototype_index + 1, self.pos),
            ])
        }
    }
}

struct State {
    player: Player,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    score: u32,
    game_over: bool,
    level: u32,
}

impl State {
    fn new(screen_bounds: Vec2) -> Self {
        let player = Player {
            pos: vec2(screen_bounds.x / 2.0, screen_bounds.y / 2.0),
            angle: 0f32,
            velocity: Vec2::ZERO,
            shot_cooldown: 0.0,
        };

        let mut asteroids = Vec::new();
        Asteroid::spawn_many(
            &mut asteroids,
            ASTEROID_STARTING_COUNT,
            screen_bounds,
            player.pos,
            PLAYER_CLEAR_RADIUS,
        );

        let bullets = Vec::new();

        Self {
            player,
            bullets,
            asteroids,
            score: 0,
            game_over: false,
            level: 1,
        }
    }
}

const PLAYER_SIZE: f32 = 10f32;
const PLAYER_CLEAR_RADIUS: f32 = PLAYER_SIZE * 25f32;

const MAX_SPEED: f32 = 1000.0f32;
const ACCELERATION: f32 = 500f32;
const FRICTION: f32 = 0.1f32;
const TURN_SPEED: f32 = 5f32;
const SHOT_COOLDOWN: f32 = 0.5f32;

const BULLET_MAX_LIFE: f32 = 0.5f32;
const BULLET_SPEED: f32 = 1000f32;
const BULLET_RADIUS: f32 = 2f32;

const ASTEROID_STARTING_COUNT: u32 = 10;

const ASTEROID_CONFIG: [AsteroidPrototype; 3] = [
    AsteroidPrototype {
        size: 60.0,
        speed: 20f32,
        score: 1,
    },
    AsteroidPrototype {
        size: 30.0,
        speed: 60f32,
        score: 2,
    },
    AsteroidPrototype {
        size: 15.0,
        speed: 180f32,
        score: 3,
    },
];

#[macroquad::main("Asteroids")]
async fn main() {
    request_new_screen_size(1024f32, 768f32);

    next_frame().await; // wait for screen resize

    let screen_bounds = vec2(screen_width(), screen_height());

    let shoot_sound = load_sound("assets/shoot.wav")
        .await
        .expect("shoot sound should be at assets/shoot.wav");
    let explosion_sound = load_sound("assets/explosion.wav")
        .await
        .expect("explosion sound should be at assets/explosion.wav");
    let death_sound = load_sound("assets/death.wav")
        .await
        .expect("death sound should be at assets/death.wav");
    let thrust_sound = load_sound("assets/thrust.wav")
        .await
        .expect("thrust sound should be at assets/thrust.wav");

    let mut state = State::new(screen_bounds);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            println!("Escape pressed, exiting...");
            break;
        }

        if state.game_over {
            state = State::new(screen_bounds);
        }

        let State {
            ref mut player,
            ref mut bullets,
            ref mut asteroids,
            ref mut score,
            ref mut game_over,
            ref mut level,
        } = state;

        let thrusting = is_key_down(KeyCode::W);
        if is_key_pressed(KeyCode::W) {
            macroquad::audio::play_sound(
                thrust_sound,
                PlaySoundParams {
                    volume: 0.5,
                    looped: true,
                },
            );
        } else if is_key_released(KeyCode::W) {
            macroquad::audio::stop_sound(thrust_sound);
        }

        let turning = if is_key_down(KeyCode::A) {
            -1.0
        } else if is_key_down(KeyCode::D) {
            1.0
        } else {
            0.0 // no turning
        };

        let shooting = is_key_down(KeyCode::S) && player.shot_cooldown <= 0.0;

        player.angle += turning * TURN_SPEED * get_frame_time();
        player.velocity += if thrusting {
            vec2(
                ACCELERATION * player.angle.cos(),
                ACCELERATION * player.angle.sin(),
            )
        } else {
            Vec2::ZERO
        } * get_frame_time();
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
        if player.pos.x > screen_bounds.x {
            player.pos.x = 0.0;
        } else if player.pos.x < 0.0 {
            player.pos.x = screen_bounds.x;
        }
        if player.pos.y > screen_bounds.y {
            player.pos.y = 0.0;
        } else if player.pos.y < 0.0 {
            player.pos.y = screen_bounds.y;
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
            play_sound_once(shoot_sound);
        }

        for bullet in bullets.iter_mut() {
            bullet.life -= get_frame_time();
            bullet.pos += bullet.velocity * get_frame_time();

            // warping
            if bullet.pos.x > screen_bounds.x {
                bullet.pos.x = 0.0;
            } else if bullet.pos.x < 0.0 {
                bullet.pos.x = screen_bounds.x;
            }
            if bullet.pos.y > screen_bounds.y {
                bullet.pos.y = 0.0;
            } else if bullet.pos.y < 0.0 {
                bullet.pos.y = screen_bounds.y;
            }
        }
        bullets.retain(|bullet| bullet.life > 0.0);

        for asteroid in asteroids.iter_mut() {
            asteroid.pos += asteroid.velocity * get_frame_time();

            // warping
            if (asteroid.pos.x + asteroid.size()) < 0.0 {
                asteroid.pos.x = screen_bounds.x + asteroid.size();
            } else if (asteroid.pos.x - asteroid.size()) > screen_bounds.x {
                asteroid.pos.x = -asteroid.size();
            }
            if (asteroid.pos.y + asteroid.size()) < 0.0 {
                asteroid.pos.y = screen_bounds.y + asteroid.size();
            } else if (asteroid.pos.y - asteroid.size()) > screen_bounds.y {
                asteroid.pos.y = -asteroid.size();
            }
        }
        let mut new_asteroids = Vec::new();
        asteroids.retain(|asteroid| {
            let mut asteroid_destroyed = false;
            bullets.retain(|bullet| {
                if (bullet.pos - asteroid.pos).length() < asteroid.size() + BULLET_RADIUS {
                    asteroid_destroyed = true;
                    *score += asteroid.score();
                    play_sound_once(explosion_sound);
                    if let Some(asteroids) = asteroid.split() {
                        new_asteroids.extend(asteroids);
                    }

                    false
                } else {
                    true
                }
            });

            if (player.pos - asteroid.pos).length() < asteroid.size() + PLAYER_SIZE {
                asteroid_destroyed = true;
                *game_over = true;
                play_sound_once(death_sound);
            }

            !asteroid_destroyed
        });
        asteroids.extend(new_asteroids);

        if asteroids.is_empty() {
            let target = (*score as f32).log2() as u32 + ASTEROID_STARTING_COUNT;
            Asteroid::spawn_many(
                asteroids,
                target,
                screen_bounds,
                player.pos,
                PLAYER_CLEAR_RADIUS,
            );
            *level += 1;
        }

        clear_background(BLACK);

        for bullet in bullets.iter() {
            draw_circle(bullet.pos.x, bullet.pos.y, BULLET_RADIUS, WHITE);
        }

        for asteroid in asteroids.iter() {
            draw_circle_lines(asteroid.pos.x, asteroid.pos.y, asteroid.size(), 1.0, WHITE);
        }

        player.draw(thrusting);

        draw_text(format!("Score: {score}").as_str(), 0.0, 32.0, 32.0, WHITE);
        draw_text(format!("Level: {level}").as_str(), 0.0, 64.0, 32.0, WHITE);
        draw_text(
            format!("Remaining: {count}", count = asteroids.len()).as_str(),
            0.0,
            screen_bounds.y,
            32.0,
            WHITE,
        );

        next_frame().await;
    }
}
