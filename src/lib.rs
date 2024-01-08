pub struct SineWave {
    frequency: f64,
    sample_rate: f64,
    time: f64,
}
impl SineWave {
    pub fn new(frequency: f64, sample_rate: f64) -> Self {
        Self {
            frequency,
            sample_rate,
            time: 0.0,
        }
    }
}
impl Iterator for SineWave {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        let value = (self.time * self.frequency * 2.0 * std::f64::consts::PI).sin();
        self.time += 1.0 / self.sample_rate;
        Some(value)
    }
}

pub struct Zeros {}
impl Iterator for Zeros {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        Some(0.0)
    }
}

pub struct ZeroOne {
    output_zero_next: bool,
}
impl ZeroOne {
    pub fn start_with_zero() -> Self {
        Self {
            output_zero_next: true,
        }
    }
    pub fn start_with_one() -> Self {
        Self {
            output_zero_next: false,
        }
    }
}

impl Iterator for ZeroOne {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        self.output_zero_next = !self.output_zero_next;
        // Logic is inverted because we toggled above.
        if self.output_zero_next {
            Some(1.0)
        } else {
            Some(0.0)
        }
    }
}

#[test]
fn test_zeros() {
    let mut zeros = Zeros {};
    assert_eq!(zeros.next(), Some(0.0));
    assert_eq!(zeros.next(), Some(0.0));
    assert_eq!(zeros.next(), Some(0.0));
}

#[test]
fn test_one_zero() {
    let mut zero_one = ZeroOne::start_with_one();
    assert_eq!(zero_one.next(), Some(1.0));
    assert_eq!(zero_one.next(), Some(0.0));
    assert_eq!(zero_one.next(), Some(1.0));
    assert_eq!(zero_one.next(), Some(0.0));

    let mut zero_one = ZeroOne::start_with_zero();
    assert_eq!(zero_one.next(), Some(0.0));
    assert_eq!(zero_one.next(), Some(1.0));
    assert_eq!(zero_one.next(), Some(0.0));
    assert_eq!(zero_one.next(), Some(1.0));
}

#[test]
fn test_sine_wave() {
    let mut sine_wave = SineWave::new(440.0, 44100.0);
    assert_eq!(sine_wave.next(), Some(0.0));
    // next sample should be greater than zero (sine wave going up)
    assert!(sine_wave.next().unwrap() > 0.0);
    // cycle enough samples to get just before zero
    for _ in 0..49 {
        assert!(sine_wave.next().unwrap() > 0.0);
    }
    // grap one more and throw it away
    _ = sine_wave.next();

    // now we go below
    // next sample should be less than zero
    assert!(sine_wave.next().unwrap() < 0.0);
}
