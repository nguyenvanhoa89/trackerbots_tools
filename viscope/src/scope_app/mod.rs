pub mod loader;
mod renderer;

use glium::{Display, Frame};

use scope_app::renderer::Renderer;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SignalType {
    Raw,
    Filtered(i64),
}

#[derive(Copy, Clone, Default)]
pub struct PlaybackState {
    pub speed: f32,
    pub offset: f32,
    pub x_zoom: f32,
    pub y_zoom: f32,
}

pub struct ScopeSource<I: Iterator<Item=f32>> {
    pub samp_rate: f32,
    pub data: I,
}

pub struct ScopeApp {
    renderer: Renderer,
    playback: PlaybackState,
}

impl ScopeApp {
    pub fn new(display: &Display) -> ScopeApp {
        ScopeApp {
            renderer: Renderer::init(&display, 0.0),
            playback: PlaybackState::default(),
        }
    }

    pub fn load<I: Iterator<Item=f32>>(&mut self, display: &Display, source: ScopeSource<I>) {
        self.renderer.reload(display, source);
    }

    pub fn draw(&self, frame: &mut Frame) {
        self.renderer.draw(self.playback, frame);
    }

    pub fn update(&mut self, dt: f32, playback: &mut PlaybackState) {
        playback.offset += dt * playback.speed / self.renderer.get_length();

        if playback.offset > 1.0 {
            playback.offset = 1.0;
            playback.speed = 0.0;
        }

        self.playback = *playback;
    }
}
