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
        .with_label("Большие ставки – большие выигрыши!")
        .center_screen();
    let widget_theme = WidgetTheme::new(ThemeType::AquaClassic);
    widget_theme.apply();

    win.set_color(Color::from_u32(0xffebee));


    let (_, inputs, mut btn) = Adder::new(); // Получаем контролы

    win.end();
    win.show();

    btn.set_callback(move |_| {
        let re = Regex::new(r"50.00;1-\(([12XХхx])\);2-\(([12XХхx])\);3-\(([12XХхx])\);4-\(([12XХхx])\);5-\(([12XХхx])\);6-\(([12XХхx])\);7-\(([12XХхx])\);8-\(([12XХхx])\);9-\(([12XХхx])\);10-\(([12XХхx])\);11-\(([12XХхx])\);12-\(([12XХхx])\);13-\(([12XХхx])\);14-\(([12XХхx])\);15-\(([12XХхx])\)")
            .unwrap();

        let goals = read_data(re); // вектор кортежей исходов
        let mut c1: String = inputs[0].value();
        if c1 == "X" || c1 == "x" || c1 == "Х" || c1 == "х" {
            c1 = "X".to_string();
        }
        let mut c2: String = inputs[1].value();
        if c2 == "X" || c2 == "x" || c2 == "Х" || c2 == "х" {
            c2 = "X".to_string();
        }
        let mut c3: String = inputs[2].value();
        if c3 == "X" || c3 == "x" || c3 == "Х" || c3 == "х" {
            c3 = "X".to_string();
        }
        let mut c4: String = inputs[3].value();
        if c4 == "X" || c4 == "x" || c4 == "Х" || c4 == "х" {
            c4 = "X".to_string();
        }
        let mut c5: String = inputs[4].value();
        if c5 == "X" || c5 == "x" || c5 == "Х" || c5 == "х" {
            c5 = "X".to_string();
        }
        let mut c6: String = inputs[5].value();
        if c6 == "X" || c6 == "x" || c6 == "Х" || c6 == "х" {
            c6 = "X".to_string();
        }
        let mut c7: String = inputs[6].value();
        if c7 == "X" || c7 == "x" || c7 == "Х" || c7 == "х" {
            c7 = "X".to_string();
        }
        let mut c8: String = inputs[7].value();
        if c8 == "X" || c8 == "x" || c8 == "Х" || c8 == "х" {
            c8 = "X".to_string();
        }
        let mut c9: String = inputs[8].value();
        if c9 == "X" || c9 == "x" || c9 == "Х" || c9 == "х" {
            c9 = "X".to_string();
        }
        let mut c10: String = inputs[9].value();
        if c10 == "X" || c10 == "x" || c10 == "Х" || c10 == "х" {
            c10 = "X".to_string();
        }
        let mut c11: String = inputs[10].value();
        if c11 == "X" || c11 == "x" || c11 == "Х" || c11 == "х" {
            c11 = "X".to_string();
        }
        let mut c12: String = inputs[11].value();
        if c12 == "X" || c12 == "x" || c12 == "Х" || c12 == "х" {
            c12 = "X".to_string();
        }
        let mut c13: String = inputs[12].value();
        if c13 == "X" || c13 == "x" || c13 == "Х" || c13 == "х" {
            c13 = "X".to_string();
        }
        let mut c14: String = inputs[13].value();
        if c14 == "X" || c14 == "x" || c14 == "Х" || c14 == "х" {
            c14 = "X".to_string();
        }
        let mut c15: String = inputs[14].value();
        if c15 == "X" || c15 == "x" || c15 == "Х" || c15 == "х" {
            c15 = "X".to_string();
        }

        let mut number_of_coincidences = 0;
        let mut n_str = 0; // номер строки в которой > 9 совпадений
        let mut numbers_str = Vec::<(usize, i32)>::new();
        for (i, goal) in goals.iter().enumerate() {
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

            if number_of_coincidences >= 9 {
                n_str += 1;
                numbers_str.push((i+1, number_of_coincidences));
            }
            number_of_coincidences = 0;
        }

        //win.set_label(&format!("{}", n_str));

        clearscreen::clear().expect("не удалось очистить консоль");
        if numbers_str.len() == 0 {
            println!("Нет совпадений");
        }
        for (n, counts) in &numbers_str {
            println!("{} совпадений в строке №{}", counts, n);
        }

        if numbers_str.len() > 0 {
            println!();
        }
        for i in 9..16 {
            if numbers_str.iter().filter(|n| n.1 == i).count() > 0 {
                println!("Число строк, в кот. совпали {} исходов: {}", i, numbers_str.iter().filter(|n| n.1 == i).count());
            }
        }
    });
    app.run().unwrap();
}

const WIN_HEIGHT: i32 = 200;
const WIN_WIDTH: i32 = 630;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_WIDTH: i32 = 30;

struct Adder {}

impl Adder {
    fn new() -> (Self, Vec<Input>, Button) {
        let mut inputs = Vec::<Input>::new();

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

        inputs.push(count1);
        inputs.push(count2);
        inputs.push(count3);
        inputs.push(count4);
        inputs.push(count5);
        inputs.push(count6);
        inputs.push(count7);
        inputs.push(count8);
        inputs.push(count9);
        inputs.push(count10);
        inputs.push(count11);
        inputs.push(count12);
        inputs.push(count13);
        inputs.push(count14);
        inputs.push(count15);

        let mut btn = Button::new(20, 80, 590, 100, "Проверить");
        btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
        let btn = btn;

        (Adder {}, inputs, btn)
    }
}

fn read_data(re: Regex) -> Vec<(String, String, String, String, String, String, String, String, String, String, String, String, String, String, String)> {
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
    goals
}