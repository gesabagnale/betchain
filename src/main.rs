use fltk::{app, prelude::*, window::Window};
use fltk_theme::{WidgetTheme, ThemeType};

fn main() {
    let app = app::App::default();


    let mut win = Window::default()
        .with_size(720, 620)
        .with_label("Большие ставки - большие выигрыши!")
        .center_screen();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();

    win.end();
    win.show();
    app.run().unwrap();
}