use std::sync::Arc;

use vst::{buffer::AudioBuffer, channels::{ChannelInfo, SpeakerArrangementType}, plugin::{Category, Info, Plugin, PluginParameters}, plugin_main, util::ParameterTransfer};
use noise_suppression_common::NoiseSuppression;

struct Parameters {
    transfer: ParameterTransfer
}

impl PluginParameters for Parameters {
    fn set_parameter(&self, index: i32, value: f32) {
        self.transfer.set_parameter(index as usize, value);
    }

    fn get_parameter(&self, index: i32) -> f32 {
        self.transfer.get_parameter(index as usize)
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "VAD Threshold (%)".to_string(),
            _ => format!("Param {}", index)
        }
    }
}

struct NoiseSuppressionMono {
    denoise: NoiseSuppression,
    parameters: Arc<Parameters>
}

impl Default for NoiseSuppressionMono {
    fn default() -> Self {
        let transfer = ParameterTransfer::new(1);
        transfer.set_parameter(0, 0.5);

        NoiseSuppressionMono {
            denoise: NoiseSuppression::default(),
            parameters: Arc::new(Parameters {
                transfer
            })
        }
    }
}

impl Plugin for NoiseSuppressionMono {
    fn get_info(&self) -> Info {
        Info {
            name: "Noise Suppression (Mono)".to_string(),
            vendor: "John Peel".to_string(),
            presets: 1,
            parameters: 1,
            inputs: 1,
            outputs: 1,
            midi_inputs: 0,
            midi_outputs: 0,
            unique_id: 0x00000190,
            version: 0x01,
            category: Category::SurroundFx,
            initial_delay: 1,
            preset_chunks: false,
            f64_precision: false,
            silent_when_stopped: true
        }
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.parameters) as Arc<dyn PluginParameters>
    }

    fn get_input_info(&self, input: i32) -> ChannelInfo {
        match input {
            0 => ChannelInfo::new(
                    "Input".to_string(),
                    Some("input".to_string()),
                    true,
                    Some(SpeakerArrangementType::Mono)
                ),
            _ => ChannelInfo::new(
                format!("Input channel {}", input),
                Some(format!("In {}", input)),
                true,
                None,
            )
        }
    }

    fn get_output_info(&self, output: i32) -> ChannelInfo {
        match output {
            0 => ChannelInfo::new(
                    "Output".to_string(),
                    Some("output".to_string()),
                    true,
                    Some(SpeakerArrangementType::Mono)
                ),
            _ => ChannelInfo::new(
                format!("Output channel {}", output),
                Some(format!("Out {}", output)),
                true,
                None,
            )
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let sample_count = buffer.samples();
        let vad_threshold = self.parameters.get_parameter(0);
        self.denoise.set_vad_threshold(vad_threshold);

        for (input, mut output) in buffer.zip() {
            self.denoise.process(&mut output, input, sample_count);
        }
    }
}

plugin_main!(NoiseSuppressionMono);