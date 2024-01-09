struct Zeros {}
impl Iterator for Zeros {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

struct Ones {}
impl Iterator for Ones {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}

struct SingleNumbers {
    ret: i32,
}
impl Iterator for SingleNumbers {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.ret)
    }
}

pub struct Incrementing {
    ret: i32,
}
impl Incrementing {
    pub fn new(start: i32) -> Self {
        Self { ret: start }
    }
}
impl Iterator for Incrementing {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.ret;
        self.ret = self.ret.overflowing_add(1).0;
        Some(r)
    }
}

pub struct AllInVec {
    v: Vec<i32>,
    index: usize,
}
impl AllInVec {
    pub fn new(v: Vec<i32>) -> Self {
        Self { v, index: 0 }
    }
}
impl Iterator for AllInVec {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.v.len() {
            None
        } else {
            let i = self.index;
            self.index += 1;
            Some(self.v[i])
        }
    }
}

pub struct AllInSlice<'a,T> {
    v: &'a [T],
    index: usize,
}
impl<'a,T> AllInSlice<'a,T> {
    pub fn new(v: &'a [T]) -> Self {
        Self { v, index: 0 }
    }
}
impl<'a,T> Iterator for AllInSlice<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.v.len() {
            None
        } else {
            let i = self.index;
            self.index += 1;
            Some(&self.v[i])
        }
    }
}


pub struct Take<I> {
    iter: I,
    count: usize,
}
impl<I> Take<I> {
    pub fn new(iter: I, count: usize) -> Self {
        Self { iter, count }
    }
}
impl<I> Iterator for Take<I>
where
    I: Iterator<Item = i32>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            self.count -= 1;
            self.iter.next()
        } else {
            None
        }
    }
}

#[test]
fn test_zeros() {
    let mut zeros = Zeros {};
    assert_eq!(zeros.next(), Some(0));
    assert_eq!(zeros.next(), Some(0));
    assert_eq!(zeros.next(), Some(0));
    assert_eq!(zeros.next(), Some(0));
}

#[test]
fn test_ones() {
    let mut ones = Ones {};
    assert_eq!(ones.next(), Some(1));
    assert_eq!(ones.next(), Some(1));
    assert_eq!(ones.next(), Some(1));
}

#[test]
fn test_numbers() {
    let mut numbers = SingleNumbers { ret: 5 };
    assert_eq!(numbers.next(), Some(5));
    assert_eq!(numbers.next(), Some(5));
    assert_eq!(numbers.next(), Some(5));

    let mut numbers = SingleNumbers { ret: 8 };
    assert_eq!(numbers.next(), Some(8));
    assert_eq!(numbers.next(), Some(8));
}

#[test]
fn test_incrementing() {
    let mut incr = Incrementing { ret: 0 };
    assert_eq!(incr.next(), Some(0));
    assert_eq!(incr.next(), Some(1));
    assert_eq!(incr.next(), Some(2));
    assert_eq!(incr.next(), Some(3));

    let mut incr = Incrementing {
        ret: std::i32::MAX - 1,
    };
    assert_eq!(incr.next(), Some(std::i32::MAX - 1));
    assert_eq!(incr.next(), Some(std::i32::MAX));
    assert_eq!(incr.next(), Some(std::i32::MIN));
}

#[test]
fn test_for_loop() {
    let incr = Incrementing::new(0);

    let mut expected = 0;
    for value in incr {
        assert_eq!(expected, value);
        expected += 1;
        if value == 10 {
            break;
        }
    }
}

#[test]
fn test_take() {
    let mut take = Take {
        iter: Zeros {},
        count: 3,
    };
    assert!(take.next().is_some());
    assert!(take.next().is_some());
    assert!(take.next().is_some());
    assert!(take.next().is_none());
}

#[test]
fn test_vec() {
    let mut iter = AllInVec::new(vec![1,2,3]);
    assert_eq!(iter.next(),Some(1));
    assert_eq!(iter.next(),Some(2));
    assert_eq!(iter.next(),Some(3));
    assert_eq!(iter.next(),None);
}

#[test]
fn test_slice() {
    let v = vec![1,2,3];
    let mut iter = AllInSlice::new(v.as_slice());
    assert_eq!(iter.next(),Some(&1));
    assert_eq!(iter.next(),Some(&2));
    assert_eq!(iter.next(),Some(&3));
    assert_eq!(iter.next(),None);

    let v = vec![1.0,2.0,3.0];
    let mut iter = AllInSlice::new(v.as_slice());
    assert_eq!(iter.next(),Some(&1.0));
    assert_eq!(iter.next(),Some(&2.0));
    assert_eq!(iter.next(),Some(&3.0));
    assert_eq!(iter.next(),None);
}