#[macro_use]
extern crate vst;

use std::sync::Arc;
use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

struct ComplexClipParams {
    threshold: AtomicFloat,
    gain: AtomicFloat,
}

impl Default for ComplexClipParams {
    fn default() -> ComplexClipParams {
        ComplexClipParams {
            threshold: AtomicFloat::new(1.0),
            gain: AtomicFloat::new(0.5),
        }
    }
}

struct ComplexClip {
    params: Arc<ComplexClipParams>,
}

impl Default for ComplexClip {
    fn default() -> ComplexClip {
        ComplexClip {
            params: Arc::new(ComplexClipParams::default()),
        }
    }
}

impl Plugin for ComplexClip {
    fn get_info(&self) -> Info {
        Info {
            name: "complex_clip".to_string(),
            vendor: "lost_guitarist_audio".to_string(),
            unique_id: 36278942,

            inputs: 2,
            outputs: 2,
            parameters: 2,
            category: Category::Effect,

            ..Info::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let threshold = self.params.threshold.get() / 3.0;
        let gain = self.params.gain.get();
        buffer.zip().for_each(|(input_buffer, output_buffer)| {
            input_buffer
                .iter()
                .zip(output_buffer)
                .for_each(|(input_sample, output_sample)| {
                    *output_sample = if *input_sample >= 0.0 {
                        (input_sample.min(threshold) / threshold) * gain
                    } else {
                        (input_sample.max(-threshold) / threshold) * gain
                    };
                });
        });
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for ComplexClipParams {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold.get(),
            1 => self.gain.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.threshold.set(value.max(0.001)),
            1 => self.gain.set(value.max(0.001)),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "threshold".to_string(),
            1 => "gain".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.threshold.get() * 100.0),
            1 => format!("{}", self.gain.get() * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 | 1 => "%".to_string(),
            _ => "".to_string(),
        }
    }
}

plugin_main!(ComplexClip);
