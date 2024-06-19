pub struct FrameTimer {
    last_tick: web_time::Instant,
    last_log: web_time::Instant,
    last_effective_frame: web_time::Instant,
    pub frame_time: f32,
    fps: f32,
}

impl Default for FrameTimer {
    fn default() -> Self {
        Self::new()
    }
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            last_tick: web_time::Instant::now(),
            last_log: web_time::Instant::now(),
            last_effective_frame: web_time::Instant::now(),
            frame_time: 0.0,
            fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        let new_instant: web_time::Instant = web_time::Instant::now();
        let elapsed_secs: f32 = (new_instant - self.last_tick).as_secs_f32();
        self.fps = 1.0 / elapsed_secs;
        self.frame_time = elapsed_secs;
        self.last_tick = new_instant;
    }

    pub fn log(&mut self) {
        let elapsed_secs: f32 = (self.last_tick - self.last_log).as_secs_f32();
        if elapsed_secs > 1.0 {
            self.last_log = web_time::Instant::now();
            log::info!(
                "Frame time {:.2}ms ({:.1} FPS)",
                self.frame_time * 1000.0,
                self.fps
            );
        }
    }

    pub fn is_it_time_to_refresh(&mut self, target_fps: f32) -> bool {
        if target_fps > 0.0 {
            (self.last_tick - self.last_effective_frame).as_secs_f32() >= 1.0 / target_fps
        } else {
            true
        }
        
    }
}