pub struct DelayLine {
    buffer: Vec<f64>,
    index: usize,
}

impl DelayLine {
    pub fn new(length: usize) -> DelayLine {
        DelayLine {
            buffer: vec![0.0, length],
            index: 0,
        }
    }

    pub fn read(&self) -> f64 {
        self.buffer[self.index]
    }

    pub fn write_and_advance(&mut self, value: f64) {
        self.buffer[self.index] = value;

        if self.index == self.buffer.len() - 1 {
            self.index = 0;
        } else {
            self.index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn length_10() {
        let mut line = super::DelayLine::new(10);
        for i in 0..10 {
            assert_eq!(line.read(), 0.0);
            line.write_and_advance(i as f64);
        }
        for i in 0..10 {
            assert_eq!(line.read(), i as f64);
            line.write_and_advance(i, 0.0);
        }
    }
}