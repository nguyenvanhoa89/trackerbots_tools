use std::error::Error;
use std::time::{Instant, Duration};

use glium::{Display, DisplayBuild, Surface, Frame, glutin};
use glium::glutin::{VirtualKeyCode, ElementState, Event, MouseButton, MouseScrollDelta, TouchPhase};

use imgui::{ImGui, Ui};
use imgui::glium_renderer::Renderer;

/// A structure for managing the mouse state
#[derive(Copy, Clone, Default)]
pub struct Mouse {
    pub in_gui: bool,
    pub pos: (f32, f32),
    pub pressed: (bool, bool, bool),
    pub wheel: f32,
}

/// A structure for managing the window state
pub struct Window {
    display: Display,
    imgui: ImGui,
    ui_renderer: Renderer,
    last_frame: Instant,

    pub mouse: Mouse,
    pub clear_color: [f32; 4],
}

impl Window {
    /// Creates a new windows instance with the specified title
    pub fn init(title: &str) -> Result<Window, Box<Error>> {
        let display = try!(glutin::WindowBuilder::new()
            .with_title(title)
            .with_vsync()
            .build_glium());

        let mut imgui = ImGui::init();
        let ui_renderer = try!(Renderer::init(&mut imgui, &display)
            .map_err(|e| format!("Unable to initialize ImGui renderer: {}", e)));

        set_imgui_meta_keys(&mut imgui);

        Ok(Window {
            display: display,
            imgui: imgui,
            ui_renderer: ui_renderer,
            last_frame: Instant::now(),
            mouse: Mouse::default(),
            clear_color: [1.0, 1.0, 1.0, 1.0],
        })
    }

    /// Gets a reference to the display
    pub fn get_display(&self) -> &Display {
        &self.display
    }

    /// Renders the window's content using the provided callback functions
    pub fn render<FUI, FOPENGL>(&mut self, mut render_ui: FUI, mut render_opengl: FOPENGL)
        -> Result<(), Box<Error>>
        where FUI: FnMut(&Ui),
              FOPENGL: FnMut(&mut Frame),
    {
        let dt = total_seconds(self.last_frame.elapsed());
        self.last_frame = Instant::now();

        self.gui_update_mouse();

        let (size_points, size_pixels) = match self.display.get_window() {
            Some(window) => {
                (window.get_inner_size_points().unwrap(), window.get_inner_size_pixels().unwrap())
            },
            None => ((1, 1), (1, 1))
        };

        // Build UI
        let ui = self.imgui.frame(size_points, size_pixels, dt as f32);
        render_ui(&ui);

        let mut target = self.display.draw();
        target.clear_color(self.clear_color[0], self.clear_color[1], self.clear_color[1],
            self.clear_color[1]);

        // Render graphics
        render_opengl(&mut target);

        self.mouse.in_gui = ui.want_capture_mouse();
        try!(self.ui_renderer.render(&mut target, ui)
            .map_err(|e| format!("Unable to render ImGui {}", e)));

        try!(target.finish());
        Ok(())
    }

    /// Reads and handles all events received by the window
    pub fn update_events(&mut self) -> bool {
        for event in self.display.poll_events() {
            match event {
                Event::Closed => return false,

                Event::KeyboardInput(state, _, Some(code)) => {
                    handle_imgui_meta_keys(&mut self.imgui, code, state == ElementState::Pressed);
                }

                Event::MouseMoved(x, y) => self.mouse.pos = (x as f32, y as f32),

                Event::MouseInput(state, MouseButton::Left) =>
                    self.mouse.pressed.0 = state == ElementState::Pressed,

                Event::MouseInput(state, MouseButton::Right) =>
                    self.mouse.pressed.1 = state == ElementState::Pressed,

                Event::MouseInput(state, MouseButton::Middle) =>
                    self.mouse.pressed.2 = state == ElementState::Pressed,

                Event::MouseWheel(MouseScrollDelta::LineDelta(_, y), TouchPhase::Moved) |
                Event::MouseWheel(MouseScrollDelta::PixelDelta(_, y), TouchPhase::Moved) =>
                    self.mouse.wheel = y,

                Event::ReceivedCharacter(c) => self.imgui.add_input_character(c),

                _ => ()
            }
        }

        true
    }

    /// Handles mouse updates for the UI
    fn gui_update_mouse(&mut self) {
        let scale = self.imgui.display_framebuffer_scale();

        self.imgui.set_mouse_pos(self.mouse.pos.0 / scale.0, self.mouse.pos.1 / scale.1);

        self.imgui.set_mouse_down(&[self.mouse.pressed.0, self.mouse.pressed.1,
            self.mouse.pressed.2, false, false]);

        self.imgui.set_mouse_wheel(self.mouse.wheel / scale.1);
    }
}

/// Computes the total number of seconds from a duration
fn total_seconds(duration: Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9
}

fn set_imgui_meta_keys(imgui: &mut ImGui) {
    use imgui::ImGuiKey::*;

    imgui.set_imgui_key(Tab, 0);
    imgui.set_imgui_key(LeftArrow, 1);
    imgui.set_imgui_key(RightArrow, 2);
    imgui.set_imgui_key(UpArrow, 3);
    imgui.set_imgui_key(DownArrow, 4);
    imgui.set_imgui_key(PageUp, 5);
    imgui.set_imgui_key(PageDown, 6);
    imgui.set_imgui_key(Home, 7);
    imgui.set_imgui_key(End, 8);
    imgui.set_imgui_key(Delete, 9);
    imgui.set_imgui_key(Backspace, 10);
    imgui.set_imgui_key(Enter, 11);
    imgui.set_imgui_key(Escape, 12);
    imgui.set_imgui_key(A, 13);
    imgui.set_imgui_key(C, 14);
    imgui.set_imgui_key(V, 15);
    imgui.set_imgui_key(X, 16);
    imgui.set_imgui_key(Y, 17);
    imgui.set_imgui_key(Z, 18);
}

fn handle_imgui_meta_keys(imgui: &mut ImGui, code: VirtualKeyCode, pressed: bool) {
    use glium::glutin::VirtualKeyCode::*;

    match code {
        Tab => imgui.set_key(0, pressed),
        Left => imgui.set_key(1, pressed),
        Right => imgui.set_key(2, pressed),
        Up => imgui.set_key(3, pressed),
        Down => imgui.set_key(4, pressed),
        PageUp => imgui.set_key(5, pressed),
        PageDown => imgui.set_key(6, pressed),
        Home => imgui.set_key(7, pressed),
        End => imgui.set_key(8, pressed),
        Delete => imgui.set_key(9, pressed),
        Back => imgui.set_key(10, pressed),
        Return => imgui.set_key(11, pressed),
        Escape => imgui.set_key(12, pressed),
        A => imgui.set_key(13, pressed),
        C => imgui.set_key(14, pressed),
        V => imgui.set_key(15, pressed),
        X => imgui.set_key(16, pressed),
        Y => imgui.set_key(17, pressed),
        Z => imgui.set_key(18, pressed),
        LControl | RControl => imgui.set_key_ctrl(pressed),
        LShift | RShift => imgui.set_key_shift(pressed),
        LAlt | RAlt => imgui.set_key_alt(pressed),
        LWin | RWin => imgui.set_key_super(pressed),
        _ => (),
    }
}