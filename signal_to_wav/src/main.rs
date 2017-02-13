//! A tool for converting raw data from the HackRF to a wav file

#[macro_use]
extern crate clap;
extern crate hound;
extern crate iq_converter;
extern crate sample;
extern crate dsp_filters;

use std::io::prelude::*;
use std::fs::File;
use std::f32;

use clap::App;

use dsp_filters::{GoertzelFilter, window};
use iq_converter::IqConverter;

use hound::{WavSpec, WavWriter};

use sample::Sample;
use sample::signal;
use sample::rate::Converter;

const FILTER_ELEMENTS: usize = 100;

fn main() {
    let matches = App::new("Signal to Wav Converter")
        .arg_from_usage("<input> 'The path to the input file.'")
        .arg_from_usage("-o, --output [output] 'Name of output file. (default: output.wav)'")
        .arg_from_usage("-s, --samp_rate [samp_rate] 'Set input file sample rate (default: 2e6)'")
        .arg_from_usage("-f, --filter [freq] 'Set filter frequency (offset in MHz)'")
        .get_matches();

    let input_filename = matches.value_of("input").unwrap();
    let output_filename = matches.value_of("output").unwrap_or("output.wav".into());
    let filter = value_t!(matches, "filter", f64);
    let input_samp_rate = value_t!(matches, "samp_rate", f64).unwrap_or(2e6);

    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16
    };

    let mut input_file = File::open(&input_filename).unwrap();
    let mut output = WavWriter::create(&output_filename, spec).unwrap();

    let mut data = vec![];
    input_file.read_to_end(&mut data).unwrap();

    if let Some(filter_freq) = filter.ok() {
        let input = IqConverter::new(data).collect::<Vec<_>>();

        let mut buffer = vec![];

        let mut filter = GoertzelFilter::new(input_samp_rate as f32, filter_freq as f32,
            FILTER_ELEMENTS, window::blackman_harris);

        for chunk in input.chunks(FILTER_ELEMENTS) {
            for &sample in chunk {
                filter.input(sample.1, sample.0);
            }
            buffer.push([filter.output()])
        }
        let max = buffer.iter().map(|x| x[0]).fold(f32::NAN, f32::max);

        let input_rate = input_samp_rate / FILTER_ELEMENTS as f64;
        let out_rate = spec.sample_rate as f64;

        let converter = Converter::from_hz_to_hz(buffer.into_iter(), input_rate, out_rate);
        let base_signal = signal::rate(out_rate).const_hz(1200.0).sine();

        for (amplitude, signal) in converter.into_iter().zip(base_signal) {
            let amp = amplitude[0] / max;
            output.write_sample((signal[0] as f32 * amp).to_sample::<i16>()).unwrap();
        }
    }
    else {
        let converter = Converter::from_hz_to_hz(IqConverter::new(data).map(|s| [s.0]),
            input_samp_rate, spec.sample_rate as f64);

        for sample in converter {
            output.write_sample(sample[0].to_sample::<i16>()).unwrap();
        }
    }
}
