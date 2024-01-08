use std::{cell::RefCell, rc::Rc};

use rust_iteartors::{ramp_down, zero, SineWave};
fn main() {
    const SAMPLE_RATE: f64 = 8000.0;

    let (mut samples, mut ping) = {
        let sine = SineWave::new(440.0, SAMPLE_RATE);
        let (ramp, ping) = CycleOnPing::new(ramp_down(1.0, SAMPLE_RATE), zero());

        (sine.zip(ramp).map(|(s, a)| s * a), ping)
    };

    let loops = (SAMPLE_RATE * 2.0) as usize;

    for sample in 0..=loops {
        println!("{sample}: {}", samples.next().unwrap());
        if sample == (SAMPLE_RATE * 1.5) as usize {
            println!("RESET");
            ping();
        }
    }
}

pub struct CycleOnPing<I1, I2> {
    iter: I1,
    iter_in_use: I1,
    when_exhausted: I2,
    reset: Rc<RefCell<bool>>,
}

impl<I1, I2> CycleOnPing<I1, I2>
where
    I1: Iterator<Item = f64> + Clone,
    I2: Iterator<Item = f64>,
{
    pub fn new(iter: I1, when_exhausted: I2) -> (Self, impl FnMut()) {
        let mut reset = Rc::new(RefCell::new(false));
        (
            Self {
                reset: reset.clone(),
                iter: iter.clone(),
                iter_in_use: iter,
                when_exhausted,
            },
            move || {
                *reset.borrow_mut() = true;
            },
        )
    }
    pub fn ping(&mut self) {
        self.iter_in_use = self.iter.clone();
    }
}
impl<I1, I2> Iterator for CycleOnPing<I1, I2>
where
    I1: Iterator<Item = f64> + Clone,
    I2: Iterator<Item = f64>,
{
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.reset.replace(false) {
            self.iter_in_use = self.iter.clone();
        }
        match self.iter_in_use.next() {
            Some(value) => Some(value),
            None => self.when_exhausted.next(),
        }
    }
}
