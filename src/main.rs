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

        let goalss = read_data(re); // вектор кортежей исходов
        let mut goals: [String; 15] = Default::default();
        for i in 0..15 {
            goals[i] = inputs[i].value();
            if goals[i] == "X" || goals[i] == "x" || goals[i] == "Х" || goals[i] == "х" {
                goals[i] = "X".to_string();
            }
        }

        let mut number_of_coincidences = 0;
        let mut n_str = 0; // номер строки в которой > 9 совпадений
        let mut numbers_str = Vec::<(usize, i32)>::new();
        for (i, goal) in goalss.iter().enumerate() {
            for j in 0..15 {
                if goal[j] == goals[j] { number_of_coincidences += 1; }
            }

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

        for i in 1..16 {
            let count = Input::new(20 + WIDGET_WIDTH*(i-1) + 10*(i-1), 35, WIDGET_WIDTH, WIDGET_HEIGHT, "");
            let _ = Frame::new(20 + WIDGET_WIDTH*(i-1) + 10*(i-1), 10, WIDGET_WIDTH, WIDGET_HEIGHT, "").set_label(&i.to_string());
            inputs.push(count);
        }

        let mut btn = Button::new(20, 80, 590, 100, "Проверить");
        btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
        let btn = btn;

        (Adder {}, inputs, btn)
    }
}

fn read_data(re: Regex) -> Vec<[String; 15]> {
    let path = "source.txt";
    let f = File::open(path);

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Не удалось открыть файл: {:?}", error),
    };

    let buffered = BufReader::new(f);

    let mut goals: Vec<[String; 15]> = Vec::new();

    for line in buffered.lines() {
        let line = &line.unwrap();
        for cap in re.captures_iter(line) {
            goals.push(
                [cap[1].to_string(), cap[2].to_string(), cap[3].to_string(), cap[4].to_string(), cap[5].to_string(), cap[6].to_string(), cap[7].to_string(), cap[8].to_string(),
                    cap[9].to_string(), cap[10].to_string(), cap[11].to_string(), cap[12].to_string(), cap[13].to_string(), cap[14].to_string(), cap[15].to_string()]
            );
        }
    }
    goals
}