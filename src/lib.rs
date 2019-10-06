#[macro_use]
extern crate vst;

use std::sync::Arc;
use vst::buffer::AudioBuffer;
use vst::plugin::{Category, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

struct ComplexClipParams {
    threshold: AtomicFloat,
    lower_threshold: AtomicFloat,
    fold: AtomicFloat,
    gain: AtomicFloat,
}

impl Default for ComplexClipParams {
    fn default() -> ComplexClipParams {
        ComplexClipParams {
            threshold: AtomicFloat::new(1.0),
            lower_threshold: AtomicFloat::new(1.0),
            fold: AtomicFloat::new(0.0),
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
            parameters: 4,
            category: Category::Effect,

            ..Info::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let threshold = self.params.threshold.get();
        let lower_threshold = self.params.lower_threshold.get();
        let fold = self.params.fold.get();
        let gain = self.params.gain.get();
        buffer.zip().for_each(|(input_buffer, output_buffer)| {
            input_buffer
                .iter()
                .zip(output_buffer)
                .for_each(|(input_sample, output_sample)| {
                    let positive = *input_sample >= 0.0;
                    let starting_value = if positive == true {
                        input_sample.min(threshold)
                    } else {
                        input_sample.max(-lower_threshold)
                    };
                    let clipped = if positive == true {
                        input_sample > &threshold
                    } else {
                        input_sample < &lower_threshold
                    };
                    *output_sample = if clipped == true {
                        if positive == true {
                            let difference = input_sample - threshold;
                            ((starting_value - (difference * fold)) / threshold) * gain
                        } else {
                            let difference = input_sample + lower_threshold;
                            ((starting_value - (difference * fold)) / lower_threshold) * gain
                        }
                    } else {
                        if positive == true {
                            (starting_value / threshold) * gain
                        } else {
                            (starting_value / lower_threshold) * gain
                        }
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
            1 => self.lower_threshold.get(),
            2 => self.fold.get(),
            3 => self.gain.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.threshold.set(value.max(0.05)),
            1 => self.lower_threshold.set(value.max(0.05)),
            2 => self.fold.set(value.min(0.50)),
            3 => self.gain.set(value.max(0.01)),
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "threshold".to_string(),
            1 => "lower_threshold".to_string(),
            2 => "fold".to_string(),
            3 => "gain".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.threshold.get() * 100.0),
            1 => format!("{}", self.lower_threshold.get() * 100.0),
            2 => format!("{}", self.fold.get() * 100.0),
            3 => format!("{}", self.gain.get() * 100.0),
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 | 1 | 2 | 3 => "%".to_string(),
            _ => "".to_string(),
        }
    }
}

plugin_main!(ComplexClip);
