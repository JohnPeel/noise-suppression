use ladspa::{DefaultValue, PROP_HARD_REALTIME_CAPABLE, PROP_INPLACE_BROKEN, Plugin, PluginDescriptor, Port, PortConnection, PortDescriptor};
use noise_suppression_common::NoiseSuppression;

struct NoiseSuppressionMono {
    denoise: NoiseSuppression
}

impl NoiseSuppressionMono {
    fn create(_: &PluginDescriptor, _: u64) -> Box<dyn Plugin + Send> {
        Box::new(NoiseSuppressionMono {
            denoise: NoiseSuppression::default()
        })
    }
}

impl Plugin for NoiseSuppressionMono {
    fn run<'a>(&mut self, sample_count: usize, ports: &[&'a PortConnection<'a>]) {
        let vad_threshold = ports[0].unwrap_control();
        let input = ports[1].unwrap_audio();
        let mut output = ports[2].unwrap_audio_mut();

        self.denoise.set_vad_threshold(*vad_threshold);
        self.denoise.process(&mut output, input, sample_count);
    }
}

struct NoiseSuppressionStereo {
    left: NoiseSuppression,
    right: NoiseSuppression
}

impl NoiseSuppressionStereo {
    fn create(_: &PluginDescriptor, _: u64) -> Box<dyn Plugin + Send> {
        Box::new(NoiseSuppressionStereo {
            left: NoiseSuppression::default(),
            right: NoiseSuppression::default()
        })
    }
}

impl Plugin for NoiseSuppressionStereo {
    fn run<'a>(&mut self, sample_count: usize, ports: &[&'a PortConnection<'a>]) {
        let vad_threshold = ports[0].unwrap_control();
        let input_left = ports[1].unwrap_audio();
        let input_right = ports[2].unwrap_audio();
        let mut output_left = ports[3].unwrap_audio_mut();
        let mut output_right = ports[4].unwrap_audio_mut();

        self.left.set_vad_threshold(*vad_threshold);
        self.right.set_vad_threshold(*vad_threshold);

        self.left.process(&mut output_left, input_left, sample_count);
        self.right.process(&mut output_right, input_right, sample_count);
    }
}

#[no_mangle]
pub fn get_ladspa_descriptor(index: u64) -> Option<PluginDescriptor> {
    match index {
        0 => Some(PluginDescriptor {
            unique_id: 0x00000190,
            label: "Noise Suppression (Mono)",
            properties: PROP_INPLACE_BROKEN | PROP_HARD_REALTIME_CAPABLE,
            name: "Noise Suppression (Mono)",
            maker: "John Peel",
            copyright: "None",
            ports: vec![
                Port {
                    name: "vad_threshold",
                    desc: PortDescriptor::ControlInput,
                    default: Some(DefaultValue::Middle),
                    lower_bound: Some(0.0),
                    upper_bound: Some(1.0),
                    ..Default::default()
                },
                Port {
                    name: "input",
                    desc: PortDescriptor::AudioInput,
                    ..Default::default()
                },
                Port {
                    name: "output",
                    desc: PortDescriptor::AudioOutput,
                    ..Default::default()
                }
            ],
            new: NoiseSuppressionMono::create,
        }),
        1 => Some(PluginDescriptor {
            unique_id: 0x00000191,
            label: "Noise Suppression (Stereo)",
            properties: PROP_INPLACE_BROKEN | PROP_HARD_REALTIME_CAPABLE,
            name: "Noise Suppression (Stereo)",
            maker: "John Peel",
            copyright: "None",
            ports: vec![
                Port {
                    name: "vad_threshold",
                    desc: PortDescriptor::ControlInput,
                    default: Some(DefaultValue::Middle),
                    lower_bound: Some(0.0),
                    upper_bound: Some(1.0),
                    ..Default::default()
                },
                Port {
                    name: "input_left",
                    desc: PortDescriptor::AudioInput,
                    ..Default::default()
                },
                Port {
                    name: "input_right",
                    desc: PortDescriptor::AudioInput,
                    ..Default::default()
                },
                Port {
                    name: "output_left",
                    desc: PortDescriptor::AudioOutput,
                    ..Default::default()
                },
                Port {
                    name: "output_right",
                    desc: PortDescriptor::AudioOutput,
                    ..Default::default()
                }
            ],
            new: NoiseSuppressionStereo::create,
        }),
        _ => None
    }
}
