#[macro_use] extern crate glium;
#[macro_use] extern crate imgui;

extern crate iq_converter;
extern crate sample;
extern crate dsp_filters;
mod window;
mod scope_app;
mod gui;

use std::error::Error;
use std::env;
use std::time::{Instant, Duration};

use window::Window;
use scope_app::{SignalType, ScopeApp, loader};
use gui::Gui;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<(), Box<Error>> {
    let mut window = try!(Window::init("ViScope"));

    let source = env::args().nth(1).unwrap_or("signal.bin".into());
    println!("Loading: {}", source);

    let mut gui = Gui::init();
    let mut app = ScopeApp::new(window.get_display());

    app.load(window.get_display(), loader::raw_data(&source));
    let mut signal_type = SignalType::Raw;

    let mut prev_time = Instant::now();
    loop {
        if gui.signal_type != signal_type {
            match gui.signal_type {
                SignalType::Raw => app.load(window.get_display(), loader::raw_data(&source)),
                SignalType::Filtered(freq) => {
                    app.load(window.get_display(),  loader::filtered_data(&source, freq as f32));
                }
            }
            signal_type = gui.signal_type;
        }

        let dt = total_seconds(prev_time.elapsed()) as f32;
        prev_time = Instant::now();

        app.update(dt, &mut gui.playback);
        try!(window.render(|ui| gui.draw(ui), |frame| app.draw(frame)));

        if !window.update_events() {
            break;
        }
    }

    Ok(())
}

/// Computes the total number of seconds from a duration
pub fn total_seconds(duration: Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9
}