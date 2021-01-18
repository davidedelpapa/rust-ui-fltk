use fltk::{app::*, button::*, frame::*, window::*};

fn main() {
    let app = App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Hello, FLTK!");
    let mut frame = Frame::new(0, 0, 400, 200, "Boring label");
    let mut but = Button::new(160, 210, 80, 40, "Click me!");

    wind.end();
    wind.show();

    // Remember: Callbacks after initializing the interface
    but.set_callback(move || frame.set_label("Hello, World!"));

    app.run().unwrap();
}
