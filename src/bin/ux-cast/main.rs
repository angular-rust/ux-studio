#![allow(unused_imports)]

use clutter::prelude::*;
use clutter::Color;
use components::prelude::*;
use components::{Stage, Spinner};
use primitives::{palette, RgbaColor};

#[derive(Default, Application)]
struct Application {
    window: Stage,
}

impl Application {
    fn new() -> Self {
        let app: Self = Default::default();
        app.window
            .set_window_size(512, 512)
            .set_title("Sample window")
            .show()
            .connect_destroy(move |_win| {
                println!("GOT {}", _win.test_check());
                Application::quit()
            });

            
        
        let color: RgbaColor = palette::TEAL_9.into();
        app.window.set_background_color(&Color::new(color.r, color.g, color.b, color.a));

        let spinner = Spinner::new();
        app.window.set_child(&spinner);
        
        app
    }
}

fn main() {
    Application::run();
}
