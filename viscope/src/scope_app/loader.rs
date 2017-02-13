use std::f32;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use std::vec;

use iq_converter::IqConverter;
use sample::rate::Converter;
use dsp_filters::GoertzelFilter;
use dsp_filters::edge_filter::EdgeFilter;
use dsp_filters::window;

use super::ScopeSource;

pub fn raw_data<P: AsRef<Path>>(file_path: P) -> ScopeSource<vec::IntoIter<f32>> {
    let mut input_file = File::open(file_path).unwrap();
    let mut data = vec![];
    input_file.read_to_end(&mut data).unwrap();

    let raw_stream = IqConverter::new(data).map(|x| [x.0]);
    let down_sampled_stream = Converter::from_hz_to_hz(raw_stream, 2e6, 1e5);
    let data: Vec<_> = down_sampled_stream.map(|x| x[0]).collect();

    ScopeSource {
        samp_rate: 1e5,
        data: data.into_iter(),
    }
}

pub fn filtered_data<P: AsRef<Path>>(file_path: P, freq: f32) -> ScopeSource<vec::IntoIter<f32>> {
    let mut edge_filter = EdgeFilter::new(20);

    let mut input_file = File::open(file_path).unwrap();
    let mut data = vec![];
    input_file.read_to_end(&mut data).unwrap();

    let input = IqConverter::new(data).collect::<Vec<_>>();
    let mut buffer = vec![];

    let window_size = 1000;
    let mut filter = GoertzelFilter::new(2e6 as f32, freq, window_size, window::blackman_harris);
    for chunk in input.chunks(window_size) {
        for &sample in chunk {
            filter.input(sample.1, sample.0);
        }
        edge_filter.input(filter.output());
        buffer.push(edge_filter.output())
    }

    ScopeSource {
        samp_rate: 2e6 / window_size as f32,
        data: buffer.into_iter(),
    }
}