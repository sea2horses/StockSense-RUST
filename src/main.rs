use crate::app::App;

mod app;

fn main() {
    let mut app = App::new();
    app.run();
}
