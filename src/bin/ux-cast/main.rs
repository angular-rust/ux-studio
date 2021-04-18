use ux::prelude::*;
use ux::{Spinner, Window};

#[derive(Default, Application)]
struct Application {
    window: Window,
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

        app.window.set_background_color(Some(color::TEAL_9));

        let spinner = Spinner::new();
        app.window.set_child(&spinner);

        app
    }
}

fn main() {
    Application::run();
}
