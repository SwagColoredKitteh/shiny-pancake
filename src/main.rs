extern crate piston_window;
extern crate window;
extern crate clap;

use clap::{Arg, App};

use std::io;

use std::sync::{Arc, Mutex};

use std::thread;

use std::str::FromStr;

use std::time::{Instant, Duration};

use std::borrow::Borrow;

use std::fs::File;

use std::io::prelude::*;

use piston_window::{ clear, ellipse, rectangle, line
                   , WindowSettings, PistonWindow
                   , PressEvent, Button, Key };

#[cfg(feature = "sdl2-backend")] extern crate sdl2_window;
#[cfg(feature = "sdl2-backend")] use sdl2_window::Sdl2Window;

#[cfg(feature = "glutin-backend")] extern crate glutin_window;
#[cfg(feature = "glutin-backend")] use glutin_window::GlutinWindow;

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
    current_frame: usize,
    play: bool,
    delay: i64,
    time_buffer: i64,
    frames: Vec<Frame>
}

impl RenderState {
    fn new() -> RenderState {
        RenderState {
            current_frame: 0,
            play: false,
            delay: 33_000_000,
            time_buffer: 0,
            frames: vec![Frame::new()]
        }
    }

    fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame]
    }

    fn current_frame_mut(&mut self) -> &mut Frame {
        let idx = self.current_frame;
        &mut self.frames[idx]
    }

    fn last_frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    fn last_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    fn new_frame(&mut self) {
        self.frames.push(Frame::new());
    }

    fn next_frame(&mut self) {
        if self.current_frame < self.frames.len() - 1 {
            self.current_frame += 1;
        }
    }

    fn prev_frame(&mut self) {
        if self.current_frame > 0 {
            self.current_frame -= 1;
        }
    }

    fn skip_frames(&mut self, amt: i64) {
        if amt > 0 {
            self.current_frame += amt as usize;
            if self.current_frame >= self.frames.len() {
                self.current_frame = self.frames.len() - 1;
            }
        }
        else if amt < 0 {
            self.current_frame = self.current_frame.saturating_sub((-amt) as usize);
        }
    }

    fn toggle_play(&mut self) {
        self.play = !self.play;
        if self.play {
            self.time_buffer = 0;
        }
    }

    fn nanos_elapsed(&mut self, nanos: i64) {
        if !self.play {
            return;
        }
        self.time_buffer += nanos;
        if self.time_buffer > self.delay {
            self.time_buffer -= self.delay;
            if self.current_frame < self.frames.len() - 1 {
                self.next_frame();
            }
            else {
                self.play = false;
                self.time_buffer = 0;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Frame {
    shapes: Vec<Shape>
}

impl Frame {
    fn new() -> Frame {
        Frame {
            shapes: Vec::new()
        }
    }
}

#[cfg(feature = "sdl2-backend")]
fn create_window(title: String, width: u32, height: u32) -> PistonWindow<Sdl2Window> {
    WindowSettings::new(title, [width, height]).build().unwrap()
}

#[cfg(feature = "glutin-backend")]
fn create_window(title: String, width: u32, height: u32) -> PistonWindow<GlutinWindow> {
    WindowSettings::new(title, [width, height]).build().unwrap()
}

fn render_thread(title: String, width: u32, height: u32, state: Arc<Mutex<RenderState>>) {
    let mut wnd = create_window(title, width, height);
    let mut timer = Instant::now();
    while let Some(e) = wnd.next() {
        wnd.draw_2d(&e, |c, g| {
            let (width, height) = {
                let vp = c.viewport.unwrap();
                (vp.draw_size[0] as i64, vp.draw_size[1] as i64)
            };
            clear([0., 0., 0., 1.], g);
            let mut guard = state.lock().unwrap();
            let new_timer = Instant::now();
            let t = new_timer.duration_since(timer).subsec_nanos() as i64;
            timer = new_timer;
            guard.nanos_elapsed(t);
            for shape in guard.current_frame().shapes.iter() {
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
            rectangle([0.2, 0.2, 0.2, 1.], [0., height as f64 - 20., width as f64, height as f64], c.transform, g);
            rectangle([0.7, 0.7, 0.7, 1.], [ 0.
                                           , height as f64 - 20.
                                           , width as f64 * (guard.current_frame as f64 / guard.frames.len() as f64)
                                           , height as f64 ], c.transform, g)
        });
        e.press(|button| {
            if let Button::Keyboard(key) = button {
                let mut guard = state.lock().unwrap();
                match key {
                    Key::Left => {
                        guard.prev_frame();
                    },
                    Key::Right => {
                        guard.next_frame();
                    },
                    Key::Return => {
                        guard.toggle_play();
                    },
                    Key::Up => {
                        guard.skip_frames(10);
                    },
                    Key::Down => {
                        guard.skip_frames(-10);
                    },
                    Key::Space => {
                        guard.toggle_play();
                    },
                    _ => ()
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
                      .arg(Arg::with_name("file")
                               .short("f")
                               .long("file")
                               .value_name("FILE")
                               .help("File to get the data from.")
                               .takes_value(true))
                      .arg(Arg::with_name("play")
                               .short("p")
                               .long("play")
                               .help("Auto-play the replay."))
                      .arg(Arg::with_name("speed")
                               .short("x")
                               .long("speed")
                               .value_name("SPEED")
                               .help("Adjust the playing speed. (as a multiple of the frame delay)")
                               .takes_value(true))
                      .get_matches();
    
    let title = matches.value_of("title").unwrap_or(env!("CARGO_PKG_NAME")).to_owned();

    let (width, height) = {
        let param = matches.value_of("size").unwrap_or("640x480");
        let mut sp = param.split("x");
        let mut ret = (
            sp.next().unwrap().parse().unwrap(),
            sp.next().unwrap().parse().unwrap()
        );
        ret.1 += 20;
        ret
    };

    let file: Option<_> = matches.value_of("file");

    let input: Box<BufRead> = if let Some(path) = file {
        let f = File::open(path).unwrap();
        Box::new(io::BufReader::new(f))
    }
    else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    let lines = input.lines().map(Result::unwrap);

    let mut rs = RenderState::new();
    
    if matches.is_present("play") {
        rs.toggle_play();
    }

    if let Some(speed_raw) = matches.value_of("speed") {
        let speed: f64 = speed_raw.parse().unwrap();
        rs.delay = (rs.delay as f64 / speed) as i64;
    }

    let my_render_state = Arc::new(Mutex::new(rs));
    let other_render_state = my_render_state.clone();

    let join_handle = thread::spawn(move || {
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
                guard.last_frame_mut().shapes.clear();
            },
            "#FRAME_START" => {
                let mut guard = my_render_state.lock().unwrap();
                guard.new_frame();
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
                guard.last_frame_mut().shapes.push(Shape::Rect(color, Vec2(x, y), Vec2(width, height)));
            },
            "#CIRCLE" => {
                let mut guard = my_render_state.lock().unwrap();
                let x: f64 = pop(&mut sp);
                let y: f64 = pop(&mut sp);
                let radius: f64 = pop(&mut sp);
                guard.last_frame_mut().shapes.push(Shape::Ellipse(color, Vec2(x - radius, y - radius), Vec2(radius * 2., radius * 2.)));
            },
            "#ELLIPSE" => {
                let mut guard = my_render_state.lock().unwrap();
                let x: f64 = pop(&mut sp);
                let y: f64 = pop(&mut sp);
                let width: f64 = pop(&mut sp);
                let height: f64 = pop(&mut sp);
                guard.last_frame_mut().shapes.push(Shape::Ellipse(color, Vec2(x, y), Vec2(width, height)));
            },
            "#LINE" => {
                let mut guard = my_render_state.lock().unwrap();
                let x1: f64 = pop(&mut sp);
                let y1: f64 = pop(&mut sp);
                let x2: f64 = pop(&mut sp);
                let y2: f64 = pop(&mut sp);
                guard.last_frame_mut().shapes.push(Shape::Line(color, Vec2(x1, y1), Vec2(x2, y2)));
            },
            "#DELAY" => {
                thread::sleep(Duration::from_millis(pop(&mut sp)));
            },
            _ => ()
        }
    }

    join_handle.join().unwrap();
}
