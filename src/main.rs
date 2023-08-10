use raylib::prelude::*;
use std::time::{Instant, Duration};
use rand::Rng;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

#[derive(Clone)]
struct Player {
    is_alive: bool,
    position: Vector2,
    velocity: Vector2,
    size: Vector2,
    color: Color,
    gravity_force: f32,
    jump_force: f32,
}

struct Pipe {
    position: Vector2,
    velocity: Vector2,
    size: Vector2,
    color: Color,
}

struct Timer {
    now: Instant,
    time: u64,
    oneshot: bool,
    stop_timer: bool,
}

impl Timer {
    pub fn new(new_time: u64, new_oneshot: bool) -> Timer {
        Timer {
            now: Instant::now(),
            time: new_time,
            oneshot: new_oneshot,
            stop_timer: false,
        }
    }

    pub fn update(&mut self, callback: unsafe fn() -> ()) {
        if self.now.elapsed() >= Duration::from_secs(self.time) {
            if self.stop_timer {
                return;
            }
            if self.oneshot {
                self.stop_timer = true;
            }

            unsafe { callback(); }
            self.now = Instant::now();
        }
    }
}

static mut PIPES: Vec<Pipe> = Vec::new();


fn main() {
    let (mut rl, thread): (RaylibHandle, RaylibThread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("FlappySquare")
        //.vsync()
        .build();

    let mut game_started: bool = false;

    let player_default_info: Player = Player {
        is_alive: true,
        position: Vector2::new(SCREEN_WIDTH as f32 / 3.0, SCREEN_HEIGHT as f32 / 2.0),
        velocity: Vector2::new(0.0, 0.0),
        size: Vector2::new(64.0, 64.0),
        color: Color::WHITE,
        gravity_force: 17.0,
        jump_force: 650.0,
    };

    let mut player: Player = player_default_info.clone();

    let mut add_pipe_timer: Timer = Timer::new(1, false);
    const PIPE_SPAWN_POS: Vector2 = Vector2::new(SCREEN_WIDTH as f32 + PIPE_SIZE.x, SCREEN_HEIGHT as f32 / 2.0);
    const PIPE_SIZE: Vector2 = Vector2::new(64.0, 64.0 * 20.0);
    const PIPE_SPEED: f32 = 300.0;

    while !rl.window_should_close() {
        let mut draw: RaylibDrawHandle = rl.begin_drawing(&thread);
        draw.clear_background(Color::BLACK);
        draw.draw_fps(30, 30);

        //Adding the player
        if player.is_alive {
            draw.draw_rectangle_v(player.position, player.size, player.color);
            player.position += player.velocity * draw.get_frame_time();
            if !game_started {
                draw.draw_text("Press 'left click' to start", SCREEN_WIDTH / 4, SCREEN_HEIGHT / 3, 50, Color::WHITE);
            } else {
                //Adding gravity
                player.velocity.y += (player.gravity_force * 110.0) * draw.get_frame_time();
            }

            //Jump
            if draw.is_key_pressed(KeyboardKey::KEY_UP)
            || draw.is_key_pressed(KeyboardKey::KEY_SPACE)
            || draw.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
                if !game_started {
                    game_started = true;
                }
                player.velocity.y = -player.jump_force;
            }

            //Killing the player
            if player.position.y <= 0.0 || player.position.y + player.size.y >= SCREEN_HEIGHT as f32 {
                player.is_alive = false;
            }
        } else {
            game_started = false;
            player = player_default_info.clone();
            unsafe { PIPES.clear(); }
        }

        //Adding the pipes
        unsafe { for pipe in 0..PIPES.len() {
            draw.draw_rectangle_v(
                Vector2::new(
                    PIPES[pipe].position.x,
                    PIPES[pipe].position.y + 120.0
                ),
                PIPES[pipe].size,
                PIPES[pipe].color
            );
            draw.draw_rectangle_v(
                Vector2::new(
                    PIPES[pipe].position.x,
                    PIPES[pipe].position.y - PIPE_SIZE.y - 120.0
                ),
                PIPES[pipe].size,
                PIPES[pipe].color
            );
            PIPES[pipe].position += PIPES[pipe].velocity * draw.get_frame_time();
        }}

        //Set add pipes timer
        if game_started {
            add_pipe_timer.update(pipe_timer_ends);
        }

        //Set pipes vector
        unsafe fn pipe_timer_ends() {
            let mut rng = rand::thread_rng();
            PIPES.push(Pipe {
                position: Vector2::new(
                    PIPE_SPAWN_POS.x,
                    PIPE_SPAWN_POS.y - rng.gen_range(-150.0..150.0)
                ),
                velocity: Vector2::new(-PIPE_SPEED, 0.0),
                size: PIPE_SIZE,
                color: Color::WHITE,
            })
        }
    }
}
