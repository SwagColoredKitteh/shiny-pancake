use frame::*;

#[derive(Clone, Debug)]
pub struct RenderState {
    current_frame: usize,
    play: bool,
    delay: i64,
    time_buffer: i64,
    frames: Vec<Frame>
}

impl RenderState {
    pub fn new() -> RenderState {
        RenderState {
            current_frame: 0,
            play: false,
            delay: 33_000_000,
            time_buffer: 0,
            frames: vec![Frame::new()]
        }
    }

    pub fn count_frames(&self) -> usize {
        self.frames.len()
    }

    pub fn current_frame_id(&self) -> usize {
        self.current_frame
    }

    pub fn current_frame(&self) -> &Frame {
        &self.frames[self.current_frame]
    }

    pub fn current_frame_mut(&mut self) -> &mut Frame {
        let idx = self.current_frame;
        &mut self.frames[idx]
    }

    pub fn last_frame(&self) -> &Frame {
        self.frames.last().unwrap()
    }

    pub fn last_frame_mut(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn new_frame(&mut self) {
        self.frames.push(Frame::new());
    }

    pub fn next_frame(&mut self) {
        if self.current_frame < self.frames.len() - 1 {
            self.current_frame += 1;
        }
    }

    pub fn prev_frame(&mut self) {
        if self.current_frame > 0 {
            self.current_frame -= 1;
        }
    }

    pub fn set_delay(&mut self, delay: i64) {
        self.delay = delay;
    }

    pub fn delay(&self) -> i64 {
        self.delay
    }

    pub fn skip_frames(&mut self, amt: i64) {
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

    pub fn toggle_play(&mut self) {
        self.play = !self.play;
        if self.play {
            self.time_buffer = 0;
        }
    }

    pub fn nanos_elapsed(&mut self, nanos: i64) {
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
