use std::collections::HashMap;

use command::*;
use render_state::*;
use color::*;

pub struct Context {
    pub color: Color,
    commands: HashMap<String, Box<Command>>
}

impl Context {
    pub fn new() -> Context {
        Context {
            color: Color(0, 0, 0, 0),
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
}
