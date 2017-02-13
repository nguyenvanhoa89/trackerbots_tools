use std::f32;

use glium::{index, Surface, Display, Frame};
use glium::{VertexBuffer, Program};

use super::{ScopeSource, PlaybackState};

#[derive(Copy, Clone)]
struct Vertex {
    pub position: [f32; 2],
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Vertex {
        Vertex {
            position: [x, y],
        }
    }
}

implement_vertex!(Vertex, position);

pub struct Renderer {
    shaders: Program,
    vertex_buffer: VertexBuffer<Vertex>,
    samp_rate: f32,
    length: f32,
}

impl Renderer {
    pub fn init(display: &Display, samp_rate: f32) -> Renderer {
        let shaders = program!(display,
            140 => {
                vertex: include_str!("../../shaders/vshader.glsl"),
                fragment: include_str!("../../shaders/fshader.glsl"),
            }
        ).unwrap();

        Renderer {
            shaders: shaders,
            vertex_buffer: VertexBuffer::new(display, &vec![]).unwrap(),
            samp_rate: samp_rate,
            length: 0.0,
        }
    }

    pub fn draw(&self, state: PlaybackState, frame: &mut Frame) {
        let uniforms = uniform! {
            dt: state.offset * self.length,
            x_scale: (1.5_f32).powf(state.x_zoom),
            y_scale: (1.5_f32).powf(state.y_zoom),
        };
        let index_buffer = index::NoIndices(index::PrimitiveType::LineStrip);

        frame.draw(&self.vertex_buffer, &index_buffer, &self.shaders, &uniforms,
            &Default::default()).unwrap();
    }

    pub fn reload<I: Iterator<Item=f32>>(&mut self, display: &Display, source: ScopeSource<I>) {
        self.samp_rate = source.samp_rate;

        let vertices: Vec<_> = source.data.enumerate()
            .map(|(x, y)| Vertex::new(x as f32 / self.samp_rate, y))
            .collect();

        self.length = vertices.len() as f32 / self.samp_rate;
        self.vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
    }

    pub fn get_length(&self) -> f32 {
        self.length
    }
}
