mod court;
mod icons;
mod term;
mod world;

use macroquad::prelude::*;

const WIDTH: u64 = 26;
const HEIGHT: u64 = 7;

fn window_conf() -> Conf {
    Conf {
        window_title: "TFTRIS".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: (WIDTH * 48) as i32,
        window_height: (HEIGHT * 48) as i32,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let icons = icons::Icons::load();
    let mut world = world::World::new(WIDTH as usize, HEIGHT as usize);

    loop {
        clear_background(WHITE);

        world.draw(&icons);
        world.update();

        next_frame().await
    }
}
