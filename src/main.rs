extern crate piston_window;
extern crate clap;

use clap::{Arg, App};

use std::io;

use std::sync::{Arc, Mutex};

use std::thread;

use std::time::Duration;

use std::str::FromStr;

use std::borrow::Borrow;

use std::io::prelude::*;

use piston_window::*;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
struct Vec2(f64, f64);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Color(u8, u8, u8, u8);

impl Color {
    fn to_arr(self) -> [f32; 4] {
        [ self.0 as f32 / 255.
        , self.1 as f32 / 255.
        , self.2 as f32 / 255.
        , self.3 as f32 / 255. ]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
enum Shape {
    Ellipse(Color, Vec2, Vec2),
    Rect(Color, Vec2, Vec2),
    Line(Color, Vec2, Vec2)
}

#[derive(Clone, Debug)]
struct RenderState {
    shapes: Vec<Shape>
}

impl RenderState {
    fn new() -> RenderState {
        RenderState {
            shapes: Vec::new()
        }
    }
}

fn render_thread(title: String, width: u32, height: u32, state: Arc<Mutex<RenderState>>) {
    let mut wnd: PistonWindow = WindowSettings::new(title, [width, height]).build().unwrap();
    while let Some(e) = wnd.next() {
        wnd.draw_2d(&e, |c, g| {
            clear([0., 0., 0., 1.], g);
            let guard = state.lock().unwrap();
            for shape in guard.shapes.iter() {
                match *shape {
                    Shape::Ellipse(col, pos, size) => {
                        ellipse(col.to_arr(), [pos.0, pos.1, size.0, size.1], c.transform, g);
                    },
                    Shape::Rect(col, pos, size) => {
                        rectangle(col.to_arr(), [pos.0, pos.1, size.0, size.1], c.transform, g);
                    },
                    Shape::Line(col, from, to) => {
                        line(col.to_arr(), 2., [from.0, from.1, to.0, to.1], c.transform, g);
                    }
                }
            }
        });
    }
}

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                      .version(env!("CARGO_PKG_VERSION"))
                      .about(env!("CARGO_PKG_DESCRIPTION"))
                      .arg(Arg::with_name("size")
                               .short("s")
                               .long("size")
                               .value_name("WIDTHxHEIGHT")
                               .help("Sets the window size. (default: 640x480)")
                               .takes_value(true))
                      .arg(Arg::with_name("title")
                               .short("t")
                               .long("title")
                               .value_name("TITLE")
                               .help("Sets the window title.")
                               .takes_value(true))
                      .get_matches();
    
    let title = matches.value_of("title").unwrap_or(env!("CARGO_PKG_NAME")).to_owned();

    let (width, height) = {
        let param = matches.value_of("size").unwrap_or("640x480");
        let mut sp = param.split("x");
        (
            sp.next().unwrap().parse().unwrap(),
            sp.next().unwrap().parse().unwrap()
        )
    };

    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let my_render_state = Arc::new(Mutex::new(RenderState::new()));
    let other_render_state = my_render_state.clone();

    thread::spawn(move || {
        render_thread(title, width, height, other_render_state);
    });
    
    let mut color = Color(0, 0, 0, 0);

    for line in lines {
        let mut sp = line.split(" ");
        fn pop<'a, T: FromStr<Err = E>, I: Iterator<Item = &'a str>, E: std::fmt::Debug>(sp: &mut I) -> T {
            sp.next().unwrap()
              .parse().unwrap()
        }
        let cmd: String = pop(&mut sp);
        match cmd.to_uppercase().borrow() {
            "#RESET" => {
                let mut guard = my_render_state.lock().unwrap();
                guard.shapes.clear();
            },
            "#COLOR" => {
                color.0 = pop(&mut sp);
                color.1 = pop(&mut sp);
                color.2 = pop(&mut sp);
                color.3 = pop(&mut sp);
            },
            "#RECT" => {
                let mut guard = my_render_state.lock().unwrap();
                let x: f64 = pop(&mut sp);
                let y: f64 = pop(&mut sp);
                let width: f64 = pop(&mut sp);
                let height: f64 = pop(&mut sp);
                guard.shapes.push(Shape::Rect(color, Vec2(x, y), Vec2(width, height)));
            },
            "#CIRCLE" => {
                let mut guard = my_render_state.lock().unwrap();
                let x: f64 = pop(&mut sp);
                let y: f64 = pop(&mut sp);
                let radius: f64 = pop(&mut sp);
                guard.shapes.push(Shape::Ellipse(color, Vec2(x - radius, y - radius), Vec2(radius * 2., radius * 2.)));
            },
            "#ELLIPSE" => {
                let mut guard = my_render_state.lock().unwrap();
                let x: f64 = pop(&mut sp);
                let y: f64 = pop(&mut sp);
                let width: f64 = pop(&mut sp);
                let height: f64 = pop(&mut sp);
                guard.shapes.push(Shape::Ellipse(color, Vec2(x, y), Vec2(width, height)));
            },
            "#LINE" => {
                let mut guard = my_render_state.lock().unwrap();
                let x1: f64 = pop(&mut sp);
                let y1: f64 = pop(&mut sp);
                let x2: f64 = pop(&mut sp);
                let y2: f64 = pop(&mut sp);
                guard.shapes.push(Shape::Line(color, Vec2(x1, y1), Vec2(x2, y2)));
            },
            "#DELAY" => {
                thread::sleep(Duration::from_millis(pop(&mut sp)));
            },
            _ => ()
        }
    }
}
