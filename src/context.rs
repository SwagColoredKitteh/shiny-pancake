use std::collections::HashMap;

use command::*;
use render_state::*;
use color::*;

pub struct Context {
    pub fill_color: Color,
    pub stroke_color: Color,
    pub stroke_width: f64,
    commands: HashMap<String, Box<Command>>
}

impl Context {
    pub fn new() -> Context {
        Context {
            fill_color: Color(0, 0, 0, 0),
            stroke_color: Color(0, 0, 0, 0),
            stroke_width: 2.,
            commands: HashMap::new()
        }
    }

    pub fn register_command(&mut self, cmd: Box<Command>) {
        self.commands.insert(cmd.name(), cmd);
    }

    pub fn execute_command(&mut self, rs: &mut RenderState, cmdstr: &str, args: Vec<String>) {
        if let Some(e) = self.commands.get(&cmdstr.to_uppercase()).map(|c| c.exec()) {
            e.execute(self, rs, args);
        }
    }

    pub fn execute_line(&mut self, rs: &mut RenderState, line: &str) {
        let mut sp = line.split(" ");
        let cmd = sp.next().unwrap();
        let mut args: Vec<String> = sp.map(|s| s.to_owned()).collect();
        args.reverse();
        self.execute_command(rs, cmd, args);
    }
}
