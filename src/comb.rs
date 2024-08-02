use super::delay_line::DelayLine;

pub struct Comb {
    delay_line: DelayLine,
    feedback: f64,
    filter_state: f64,
    dampening: f64,
    dampening_inverse: f64,
}

impl Comb {
    pub fn new(delay_length: usize) -> Comb {
        Comb {
            delay_line: DelayLine::new(delay_length),
            feedback: 0.0,
            filter_state: 0.0,
            dampening: 0.0,
            dampening_inverse: 0.0,
        }
    }

    pub fn tick(&mut self, input: f64) -> f64 {
        let output = self.delay_line.read();

        self.filter_state = output * self.dampening_inverse + self.filter_state * self.dampening;

        self.delay_line.write_and_advance(input + self.filter_state *self.feedback);

        output
    }
}