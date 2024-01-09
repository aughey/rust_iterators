pub struct Value {
    value: i32,
}
impl Value {
    pub fn new(value: i32) -> Value {
        Value { value }
    }
}

impl Iterator for Value {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }
}

pub struct Counter {
    count: i32,
}
impl Counter {
    pub fn new(start: i32) -> Counter {
        Counter { count: start }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.count;
        self.count = self.count.overflowing_add(1).0;
        Some(ret)
    }
}

pub struct VecIterator {
    vec: Vec<i32>,
    index: usize,
}
impl VecIterator {
    pub fn new(vec: Vec<i32>) -> VecIterator {
        VecIterator { vec, index: 0 }
    }
}
impl Iterator for VecIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.vec.len() {
            None
        } else {
            let ret = self.vec[self.index];
            self.index += 1;
            Some(ret)
        }
    }
}

#[test]
fn test_value() {
    let mut z = Value::new(2);
    assert_eq!(z.next(), Some(2));
    assert_eq!(z.next(), Some(2));
    assert_eq!(z.next(), Some(2));
}

#[test]
fn test_count() {
    let mut z = Counter::new(2);
    assert_eq!(z.next(), Some(2));
    assert_eq!(z.next(), Some(3));
    assert_eq!(z.next(), Some(4));
    assert_eq!(z.next(), Some(5));
}

#[test]
fn test_vec() {
    let mut z = VecIterator::new(vec![1, 2, 3]);
    assert_eq!(z.next(), Some(1));
    assert_eq!(z.next(), Some(2));
    assert_eq!(z.next(), Some(3));
    assert_eq!(z.next(), None);
}
