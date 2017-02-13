use imgui::*;

use scope_app::{PlaybackState, SignalType};

pub struct Gui {
    pub playback: PlaybackState,
    pub signal_type: SignalType,
    filter_freq: f32,
}

impl Gui {
    pub fn init() -> Gui {
        Gui {
            playback: PlaybackState::default(),
            signal_type: SignalType::Raw,
            filter_freq: 130000.0,
        }
    }

    pub fn draw(&mut self, ui: &Ui) {
        ui.window(im_str!("Playback"))
            .movable(true)
            .size((500.0, 100.0), ImGuiSetCond_FirstUseEver)
            .build(|| draw_window(self, ui))
    }
}

fn draw_window(gui: &mut Gui, ui: &Ui) {
    ui.slider_float(im_str!("Playback Speed"), &mut gui.playback.speed, 0.0, 2.0).build();
    ui.slider_float(im_str!("Playback Offset"), &mut gui.playback.offset, 0.0, 1.0).build();

    ui.separator();

    ui.slider_float(im_str!("Vertical Zoom"), &mut gui.playback.y_zoom, -10.0, 10.0).build();
    ui.slider_float(im_str!("Horizontal Zoom"), &mut gui.playback.x_zoom, -10.0, 20.0).build();

    ui.separator();

    if ui.small_button(im_str!("Raw signal")) {
        gui.signal_type = SignalType::Raw;
    }

    ui.same_line(100.0);

    if ui.small_button(im_str!("Filtered signal")) {
        gui.signal_type = SignalType::Filtered(gui.filter_freq as i64)
    }

    ui.input_float(im_str!("Filter freq"), &mut gui.filter_freq).build();

    if gui.filter_freq < -1e6 {
        gui.filter_freq = -1e6;
    }
    else if gui.filter_freq > 1e6 {
        gui.filter_freq = 1e6;
    }
}