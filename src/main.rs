use fltk::{app, prelude::*, window::Window, button::Button, input::Input};
use fltk::frame::Frame;
use fltk_theme::{WidgetTheme, widget_themes, ThemeType};


fn main() {
    let app = app::App::default();

    let mut win = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label("Большие ставки - большие выигрыши!")
        .center_screen();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();

    let (_, _textboxs) = Adder::new(); // Получаем контролы

    win.end();
    win.show();
    app.run().unwrap();
}

const WIN_HEIGHT: i32 = 200;
const WIN_WIDTH: i32 = 630;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_WIDTH: i32 = 30;

struct Adder {}

impl Adder {
    fn new() -> (Self, Vec::<Input>) {
        let mut textboxs = Vec::<Input>::new();

        let count1 = Input::new(20, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "1");

        let count2 = Input::new(20+WIDGET_WIDTH + 10, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH + 10, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "2");

        let count3 = Input::new(20+WIDGET_WIDTH*2 + 10*2, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*2 + 10*2, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "3");

        let count4 = Input::new(20+WIDGET_WIDTH*3 + 10*3, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*3 + 10*3, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "4");

        let count5 = Input::new(20+WIDGET_WIDTH*4 + 10*4, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*4 + 10*4, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "5");

        let count6 = Input::new(20+WIDGET_WIDTH*5 + 10*5, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*5 + 10*5, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "6");

        let count7 = Input::new(20+WIDGET_WIDTH*6 + 10*6, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*6 + 10*6, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "7");

        let count8 = Input::new(20+WIDGET_WIDTH*7 + 10*7, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*7 + 10*7, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "8");

        let count9 = Input::new(20+WIDGET_WIDTH*8 + 10*8, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*8 + 10*8, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "9");

        let count10 = Input::new(20+WIDGET_WIDTH*9 + 10*9, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*9 + 10*9, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "10");

        let count11 = Input::new(20+WIDGET_WIDTH*10 + 10*10, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*10 + 10*10, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "11");

        let count12 = Input::new(20+WIDGET_WIDTH*11 + 10*11, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*11 + 10*11, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "12");

        let count13 = Input::new(20+WIDGET_WIDTH*12 + 10*12, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*12 + 10*12, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "13");

        let count14 = Input::new(20+WIDGET_WIDTH*13 + 10*13, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*13 + 10*13, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "14");

        let count15 = Input::new(20+WIDGET_WIDTH*14 + 10*14, 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
        let _ = Frame::new(20+WIDGET_WIDTH*14 + 10*14, 10, WIDGET_WIDTH, WIDGET_HEIGHT, "15");

        textboxs.push(count1);
        textboxs.push(count2);
        textboxs.push(count3);
        textboxs.push(count4);
        textboxs.push(count5);
        textboxs.push(count6);
        textboxs.push(count7);
        textboxs.push(count8);
        textboxs.push(count9);
        textboxs.push(count10);
        textboxs.push(count11);
        textboxs.push(count12);
        textboxs.push(count13);
        textboxs.push(count14);
        textboxs.push(count15);

        let mut btn = Button::new(20, 80, 590, 100, "Проверить");
        btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);

        (Adder {}, textboxs)
    }
}