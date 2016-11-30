use context::*;
use render_state::*;

pub trait Command {
    fn name(&self) -> String;
    fn exec(&self) -> Box<Executor>;
}

pub trait Executor {
    fn execute(&self, context: &mut Context, state: &mut RenderState, args: Vec<String>);
}

macro_rules! define_command {
    ($name:expr, $f:ident($($t:ty),*)) => {{
        #[derive(Copy, Clone)]
        struct Cmd;
        struct Exec;

        impl Command for Cmd {
            fn name(&self) -> String { $name.to_owned() }

            fn exec(&self) -> Box<Executor> {
                Box::new(Exec)
            }
        }

        impl Executor for Exec {
            #[allow(unused_mut)]
            #[allow(unused_variables)]
            fn execute(&self, ctx: &mut Context, state: &mut RenderState, mut args: Vec<String>) {
                $f(ctx, state, $(args.pop().unwrap().parse::<$t>().unwrap(),)*);
            }
        }

        Cmd
    }}
}

macro_rules! register_command {
    ($ctx:ident, $name:expr, $f:ident($($t:ty),*)) => {{
        let cmd = define_command!($name, $f($($t),*));
        $ctx.register_command(Box::new(cmd));
    }}
}
