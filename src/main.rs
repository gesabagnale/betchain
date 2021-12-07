#![windows_subsystem="windows"]
use std::fs::File;
use std::io::{BufRead, BufReader};
use fltk::{app, prelude::*, window::Window, button::Button, input::Input};
use fltk::enums::Color;
use fltk::frame::Frame;
use fltk_theme::{WidgetTheme, widget_themes, ThemeType};
use regex::Regex;

fn main() {
    let app = app::App::default();

    let mut win = Window::default()
        .with_size(WIN_WIDTH, WIN_HEIGHT)
        .with_label("Большие ставки - большие выигрыши!")
        .center_screen();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();

    win.set_color(Color::from_u32(0xffebee));


    let (_, textboxs, mut btn) = Adder::new(); // Получаем контролы

    // Регулярка
    let re = Regex::new(r"50.00;1-\(([12XХхx])\);2-\(([12XХхx])\);3-\(([12XХхx])\);4-\(([12XХхx])\);5-\(([12XХхx])\);6-\(([12XХхx])\);7-\(([12XХхx])\);8-\(([12XХхx])\);9-\(([12XХхx])\);10-\(([12XХхx])\);11-\(([12XХхx])\);12-\(([12XХхx])\);13-\(([12XХхx])\);14-\(([12XХхx])\);15-\(([12XХхx])\)")
        .unwrap();

    // Считываем исходные данные из файла
    let path = "source.txt";
    let f = File::open(path);

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Не удалось открыть файл: {:?}", error),
    };

    let buffered = BufReader::new(f);

    let mut goals: Vec<(String, String, String, String, String, String, String, String, String, String, String, String, String, String, String)> = Vec::new();

    for line in buffered.lines() {
        let line = &line.unwrap();
        for cap in re.captures_iter(line) {
            goals.push((cap[1].to_string(), cap[2].to_string(), cap[3].to_string(), cap[4].to_string(), cap[5].to_string(), cap[6].to_string(), cap[7].to_string(),
                cap[8].to_string(), cap[9].to_string(), cap[10].to_string(), cap[11].to_string(), cap[12].to_string(), cap[13].to_string(), cap[14].to_string(), cap[15].to_string()));
        }
    }

    win.end();
    win.show();

    btn.set_callback(move |_| {
        let c1: String = textboxs[0].value();
        let c2: String = textboxs[1].value();
        let c3: String = textboxs[2].value();
        let c4: String = textboxs[3].value();
        let c5: String = textboxs[4].value();
        let c6: String = textboxs[5].value();
        let c7: String = textboxs[6].value();
        let c8: String = textboxs[7].value();
        let c9: String = textboxs[8].value();
        let c10: String = textboxs[9].value();
        let c11: String = textboxs[10].value();
        let c12: String = textboxs[11].value();
        let c13: String = textboxs[12].value();
        let c14: String = textboxs[13].value();
        let c15: String = textboxs[14].value();

        let mut n_str = 0;
        let mut number_of_coincidences = 0;

        for goal in &goals {

            if goal.0 == c1 { number_of_coincidences += 1; }
            if goal.1 == c2 { number_of_coincidences += 1; }
            if goal.2 == c3 { number_of_coincidences += 1; }
            if goal.3 == c4 { number_of_coincidences += 1; }
            if goal.4 == c5 { number_of_coincidences += 1; }
            if goal.5 == c6 { number_of_coincidences += 1; }
            if goal.6 == c7 { number_of_coincidences += 1; }
            if goal.7 == c8 { number_of_coincidences += 1; }
            if goal.8 == c9 { number_of_coincidences += 1; }
            if goal.9 == c10 { number_of_coincidences += 1; }
            if goal.10 == c11 { number_of_coincidences += 1; }
            if goal.11 == c12 { number_of_coincidences += 1; }
            if goal.12 == c13 { number_of_coincidences += 1; }
            if goal.13 == c14 { number_of_coincidences += 1; }
            if goal.14 == c15 { number_of_coincidences += 1; }

            if number_of_coincidences >= 9 { n_str += 1; }
            number_of_coincidences = 0;
        }

        win.set_label(&format!("{}", n_str));


    });
    app.run().unwrap();
}

const WIN_HEIGHT: i32 = 200;
const WIN_WIDTH: i32 = 630;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_WIDTH: i32 = 30;

struct Adder {}

impl Adder {
    fn new() -> (Self, Vec::<Input>, Button) {
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
        let btn = btn;

        (Adder {}, textboxs, btn)
    }
}