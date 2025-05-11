use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Snake".to_owned(),
        window_width: 500,
        window_height: 500,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}
