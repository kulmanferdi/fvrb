pub struct DelayLine {
    buffer: Vec<f64>,
    index: usize,
}

impl DelayLine {
    pub fn new(length: usize) -> DelayLine {
        DelayLine{
            buffer: vec![0.0, length],
            index: 0,
        }
    }
}