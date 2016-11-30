use std::time::Instant;

use std::sync::{Arc, Mutex};

use piston_window::{ clear, rectangle, ellipse, Rectangle, Ellipse, line
                   , WindowSettings, PistonWindow
                   , PressEvent, Button, Key };

use render_state::*;
use shape::*;

#[cfg(feature = "sdl2-backend")] extern crate sdl2_window;
#[cfg(feature = "sdl2-backend")] use self::sdl2_window::Sdl2Window;

#[cfg(feature = "glutin-backend")] extern crate glutin_window;
#[cfg(feature = "glutin-backend")] use self::glutin_window::GlutinWindow;

#[cfg(feature = "sdl2-backend")]
pub fn create_window(title: String, width: u32, height: u32) -> PistonWindow<Sdl2Window> {
    WindowSettings::new(title, [width, height]).build().unwrap()
}

#[cfg(feature = "glutin-backend")]
pub fn create_window(title: String, width: u32, height: u32) -> PistonWindow<GlutinWindow> {
    WindowSettings::new(title, [width, height]).build().unwrap()
}

pub fn render_thread(title: String, width: u32, height: u32, state: Arc<Mutex<RenderState>>) {
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
                    Shape::Ellipse(fill_col, border_col, stroke_width, pos, size) => {
                        let mut elli = Ellipse::new(fill_col.to_arr());
                        if border_col.3 > 0 {
                            elli = elli.border(ellipse::Border{ color: border_col.to_arr(), radius: stroke_width });
                        }
                        elli.draw([pos.0, pos.1, size.0, size.1], &Default::default(), c.transform, g);
                    },
                    Shape::Rect(fill_col, border_col, stroke_width, pos, size) => {
                        let mut rect = Rectangle::new(fill_col.to_arr());
                        if border_col.3 > 0 {
                            rect = rect.border(rectangle::Border { color: border_col.to_arr(), radius: stroke_width });
                        }
                        rect.draw([pos.0, pos.1, size.0, size.1], &Default::default(), c.transform, g);
                    },
                    Shape::Line(col, stroke_width, from, to) => {
                        line(col.to_arr(), stroke_width, [from.0, from.1, to.0, to.1], c.transform, g);
                    }
                }
            }
            rectangle([0.2, 0.2, 0.2, 1.], [0., height as f64 - 20., width as f64, height as f64], c.transform, g);
            rectangle([0.7, 0.7, 0.7, 1.], [ 0.
                                           , height as f64 - 20.
                                           , width as f64 * (guard.current_frame_id() as f64 / (guard.count_frames() - 1) as f64)
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
