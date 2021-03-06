
use dasp_sample::Sample;
use nnnoiseless::DenoiseState;

pub const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;

pub struct NoiseSuppression {
    first: bool,
    state: Box<DenoiseState<'static>>,
    input_buffer: Vec<f32>,
    output_buffer: Vec<f32>,
    vad_threshold: f32,
    initial_grace_periods: usize,
    remaining_grace_periods: usize
}

impl Default for NoiseSuppression {
    fn default() -> Self {
        NoiseSuppression {
            first: true,
            state: DenoiseState::new(),
            input_buffer: Vec::with_capacity(FRAME_SIZE * 2),
            output_buffer: Vec::with_capacity(FRAME_SIZE * 2),
            vad_threshold: 0.5,
            initial_grace_periods: 20,
            remaining_grace_periods: 0
        }
    }
}

impl NoiseSuppression {
    pub fn set_grace_periods(&mut self, grace_periods: usize) {
        self.initial_grace_periods = grace_periods;
        self.remaining_grace_periods = usize::min(self.remaining_grace_periods, self.initial_grace_periods);
    }

    pub fn set_vad_threshold(&mut self, vad_threshold: f32) {
        self.vad_threshold = vad_threshold;
    }

    pub fn process(&mut self, output: &mut [f32], input: &[f32], sample_count: usize) {
        self.input_buffer.extend(input.iter()
            .map(|f| i16::from_sample(*f) as f32));

        let samples_to_process = self.input_buffer.len() / FRAME_SIZE;
        let frames_to_process = samples_to_process * FRAME_SIZE;

        let output_start = self.output_buffer.len();
        self.output_buffer.extend(vec![f32::EQUILIBRIUM; frames_to_process]);

        for i in 0..samples_to_process {
            let output_buffer = &mut self.output_buffer[output_start + i * FRAME_SIZE .. output_start + i * FRAME_SIZE + FRAME_SIZE];
            let vad_probability = {
                let input_buffer = &self.input_buffer[0 .. FRAME_SIZE];
                self.state.process_frame(output_buffer, input_buffer)
            };
            self.input_buffer.drain(0 .. FRAME_SIZE);

            if !self.first {
                if vad_probability >= self.vad_threshold {
                    self.remaining_grace_periods = self.initial_grace_periods;
                }

                if self.remaining_grace_periods > 0 {
                    self.remaining_grace_periods -= 1;
                    output_buffer.iter_mut().for_each(|f| *f = f32::from_sample(*f as i16));
                } else {
                    output_buffer.iter_mut().for_each(|f| *f = f32::EQUILIBRIUM);
                }
            } else {
                output_buffer.iter_mut().for_each(|f| *f = f32::EQUILIBRIUM);
                self.first = false;
            }
        }

        let frames_to_copy = usize::min(self.output_buffer.len(), sample_count);
        output[0 .. frames_to_copy].copy_from_slice(&self.output_buffer[0 .. frames_to_copy]);
        self.output_buffer.drain(0 .. frames_to_copy);

        if frames_to_copy < sample_count {
            output[frames_to_copy .. sample_count].iter_mut()
                .for_each(|f| *f = f32::EQUILIBRIUM);
        }
    }
}