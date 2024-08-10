use lv2::prelude::*;
use noise_suppression_common::NoiseSuppression;

#[derive(PortCollection)]
struct PortsMono {
    vad_threshold: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

#[derive(PortCollection)]
struct PortsStereo {
    vad_threshold: InputPort<Control>,
    left_input: InputPort<Audio>,
    right_input: InputPort<Audio>,
    left_output: OutputPort<Audio>,
    right_output: OutputPort<Audio>,
}

#[uri("urn:johnpeel:noise_suppression#mono")]
struct NoiseSuppressionMono {
    denoise: NoiseSuppression,
}

#[uri("urn:johnpeel:noise_suppression#stereo")]
struct NoiseSuppressionStereo {
    left: NoiseSuppression,
    right: NoiseSuppression,
}

impl Plugin for NoiseSuppressionMono {
    type Ports = PortsMono;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self {
            denoise: NoiseSuppression::default(),
        })
    }

    fn run(&mut self, ports: &mut PortsMono, _features: &mut (), sample_count: u32) {
        let sample_count = sample_count as usize;
        if sample_count == 0 {
            return;
        }

        self.denoise.set_vad_threshold(*ports.vad_threshold);
        self.denoise
            .process(&mut ports.output, &ports.input, sample_count);
    }
}

impl Plugin for NoiseSuppressionStereo {
    type Ports = PortsStereo;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        Some(Self {
            left: NoiseSuppression::default(),
            right: NoiseSuppression::default(),
        })
    }

    fn run(&mut self, ports: &mut PortsStereo, _features: &mut (), sample_count: u32) {
        let sample_count = sample_count as usize;
        if sample_count == 0 {
            return;
        }

        self.left.set_vad_threshold(*ports.vad_threshold);
        self.right.set_vad_threshold(*ports.vad_threshold);

        self.left
            .process(&mut ports.left_output, &ports.left_input, sample_count);
        self.right
            .process(&mut ports.right_output, &ports.right_input, sample_count);
    }
}

lv2_descriptors!(NoiseSuppressionMono, NoiseSuppressionStereo);
