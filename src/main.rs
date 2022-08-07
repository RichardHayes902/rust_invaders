use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 60f32;
const PLAYER_SPEED: f32 = 600f32;

struct Player {
    rect: Rect
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE * 0.5f32,
                PLAYER_SIZE,
                PLAYER_SIZE,
                PLAYER_SIZE
            )
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32
        };
        self.rect.x += x_move * dt * PLAYER_SPEED;

        let y_move = match (is_key_down(KeyCode::Up), is_key_down(KeyCode::Down)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32
        };
        self.rect.y += y_move * dt * PLAYER_SPEED;

        // x-edge collision
        if self.rect.x >= screen_width() - PLAYER_SIZE {
            self.rect.x = screen_width() - PLAYER_SIZE
        }
        if self.rect.x < 0f32 {
            self.rect.x = 0f32
        }

        // y-edge collision
        if self.rect.y >= screen_height() - PLAYER_SIZE {
            self.rect.y = screen_height() - PLAYER_SIZE
        }
        if self.rect.y < PLAYER_SIZE {
            self.rect.y = PLAYER_SIZE
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

fn rust_invaders() -> Conf {
    Conf {
        window_title: "Rust Invaders".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(rust_invaders)]
async fn main() {
    let mut player = Player::new();
    loop {
        clear_background(WHITE);
        player.draw();
        player.update(get_frame_time());
        next_frame().await;
    }
}
