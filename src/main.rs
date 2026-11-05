mod background;
mod bar;

use amane::*;

fn main() {
    Palette::write().open(&background::BACKGROUND_PATH, 16);

    App::new().window_per_monitor(bar::view).run();
}
