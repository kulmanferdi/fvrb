use super::delay_line::DelayLine;

pub struct AllPass {
    delay_line: DelayLine,
}

impl AllPass {
    pub fn new(delay_length: usize) -> AllPass {
        AllPass {
            delay_line: DelayLine::new(delay_length),
        }
    }

    pub fn tick(&mut self, input: f64) -> f64 {
        let delayed = self.delay_line.read();
        let output = -input + delayed;
        let feedback = 0.5;

        self.delay_line.write_and_advance(input + delayed * feedback);

        output
    }
}