extern crate piston_window;
extern crate window;
extern crate clap;

use clap::{Arg, App};

use std::io;

use std::thread;

use std::fs::File;

use std::sync::{Arc, Mutex};

use std::io::prelude::*;

#[macro_use] mod command;
mod context;
mod vec2;
mod color;
mod shape;
mod render_state;
mod renderer;
mod frame;

use color::*;
use vec2::*;
use command::*;
use context::*;
use shape::*;
use render_state::*;
use renderer::*;

fn cmd_frame_start(_: &mut Context, state: &mut RenderState) {
    state.new_frame();
}

fn cmd_circle(ctx: &mut Context, state: &mut RenderState, x: f64, y: f64, radius: f64) {
    state.last_frame_mut().shapes.push(Shape::Ellipse(ctx.fill_color, ctx.stroke_color, ctx.stroke_width, Vec2(x - radius, y - radius), Vec2(radius * 2., radius * 2.)));
}

fn cmd_ellipse(ctx: &mut Context, state: &mut RenderState, x: f64, y: f64, w: f64, h: f64) {
    state.last_frame_mut().shapes.push(Shape::Ellipse(ctx.fill_color, ctx.stroke_color, ctx.stroke_width, Vec2(x, y), Vec2(w, h)));
}

fn cmd_rect(ctx: &mut Context, state: &mut RenderState, x: f64, y: f64, w: f64, h: f64) {
    state.last_frame_mut().shapes.push(Shape::Rect(ctx.fill_color, ctx.stroke_color, ctx.stroke_width, Vec2(x, y), Vec2(w, h)));
}

fn cmd_fill_color(ctx: &mut Context, _: &mut RenderState, r: u8, g: u8, b: u8, a: u8) {
    ctx.fill_color = Color(r, g, b, a);
}

fn cmd_stroke_color(ctx: &mut Context, _: &mut RenderState, r: u8, g: u8, b: u8, a: u8) {
    ctx.stroke_color = Color(r, g, b, a);
}

fn cmd_stroke_width(ctx: &mut Context, _: &mut RenderState, width: f64) {
    ctx.stroke_width = width;
}

fn cmd_line(ctx: &mut Context, state: &mut RenderState, x1: f64, y1: f64, x2: f64, y2: f64) {
    state.last_frame_mut().shapes.push(Shape::Line(ctx.stroke_color, ctx.stroke_width, Vec2(x1, y1), Vec2(x2, y2)));
}

fn cmd_nofill(ctx: &mut Context, _: &mut RenderState) {
    ctx.fill_color = Color(0, 0, 0, 0);
}

fn cmd_nostroke(ctx: &mut Context, _: &mut RenderState) {
    ctx.stroke_color = Color(0, 0, 0, 0);
}

fn add_default_commands(ctx: &mut Context) {
    register_command!(ctx, "#FRAME_START", cmd_frame_start());
    register_command!(ctx, "#STROKE_COLOR", cmd_stroke_color(u8, u8, u8, u8));
    register_command!(ctx, "#FILL_COLOR", cmd_fill_color(u8, u8, u8, u8));
    register_command!(ctx, "#STROKE_WIDTH", cmd_stroke_width(f64));
    register_command!(ctx, "#NOFILL", cmd_nofill());
    register_command!(ctx, "#NOSTROKE", cmd_nostroke());
    register_command!(ctx, "#CIRCLE", cmd_circle(f64, f64, f64));
    register_command!(ctx, "#ELLIPSE", cmd_ellipse(f64, f64, f64, f64));
    register_command!(ctx, "#RECT", cmd_rect(f64, f64, f64, f64));
    register_command!(ctx, "#LINE", cmd_line(f64, f64, f64, f64));
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
        let delay = rs.delay();
        rs.set_delay((delay as f64 / speed) as i64);
    }

    let my_render_state = Arc::new(Mutex::new(rs));
    let other_render_state = my_render_state.clone();
    
    let mut ctx = Context::new();
    add_default_commands(&mut ctx);

    let join_handle = thread::spawn(move || {
        render_thread(title, width, height, other_render_state);
    });
    
    for line in lines {
        let mut guard = my_render_state.lock().unwrap();
        ctx.execute_line(&mut guard, &line);
    }

    join_handle.join().unwrap();
}
