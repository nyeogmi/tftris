use super::court::Court;
use crate::icons::Icons;
use super::term::TermGenerator;
use macroquad::prelude::*;

pub(crate) struct World {
    court: Court,
    termgen: TermGenerator,

    frame: u64,
    update_cooldown: u64,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        World {
            court: Court::new(width, height),
            termgen: TermGenerator::new(),

            frame: 0,
            update_cooldown: 0,
        }
    }

    pub fn draw(&mut self, icons: &Icons) {
        self.court.draw_grid(
            |x, y, glyph| { 
                draw_icon(x as f32 * 48.0, y as f32 * 48.0, icons.pick(glyph)) 
            }
        )
    }

    pub fn update(&mut self) {
        self.frame += 1;

        let mut dy = 0;
        if is_key_pressed(KeyCode::Up) { dy -= 1 }
        if is_key_pressed(KeyCode::Down) { dy += 1 }
        self.court.move_y(dy);

        if is_key_pressed(KeyCode::Right) { self.court.fallfast() }

        if self.update_cooldown == 0 {
            self.court.update(|| self.termgen.generate_term());
            self.update_cooldown = 25;
        } 
        self.update_cooldown -= 1;
    }
}


fn draw_icon(x: f32, y: f32, icon: Texture2D) {
    let mut params = DrawTextureParams::default();
    draw_rectangle_lines(x, y, 48.0, 48.0, 1.0, BLACK);
    draw_rectangle_lines(x + 1.0, y + 1.0, 46.0, 46.0, 1.0, WHITE);
    params.dest_size = Some(vec2(44.0, 44.0));
    draw_texture_ex(icon, x + 2.0, y + 2.0, WHITE, params);
}