use macroquad::prelude::*;

const PLAYER_SIZE: f32 = 60f32;
const PLAYER_SPEED: f32 = 600f32;
const BOMB_SIZE: f32 = 15f32;
const BOMB_SPEED: f32 = 600f32;

// player
struct Player {
    rect: Rect,
    projectiles: Vec<Bomb>
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE * 0.5f32,
                PLAYER_SIZE,
                PLAYER_SIZE,
                PLAYER_SIZE
            ),
            projectiles: Vec::new()
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

        // track bombs
        for projectile in self.projectiles.iter_mut() {
            projectile.update(get_frame_time())
        }

        self.projectiles.retain(|projectile| projectile.rect.y < screen_height())
    }

    pub fn fire(&mut self) {
        let bomb = Bomb::new(vec2(
            self.rect.x + PLAYER_SIZE * 0.5f32 - BOMB_SIZE * 0.5f32,
            self.rect.y
        ));
        self.projectiles.push(bomb);
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}

// bombs
struct Bomb {
    rect: Rect,
    vel: Vec2
}

impl Bomb {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(
                pos.x,
                pos.y,
                BOMB_SIZE,
                BOMB_SIZE
            ),
            vel: vec2(pos.x, -1f32)
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.y += self.vel.y * dt * BOMB_SPEED;
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
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

        if is_key_pressed(KeyCode::Space) {
            player.fire()
        }

        for projectile in player.projectiles.iter() {
            projectile.draw()
        }

        next_frame().await;
    }
}
