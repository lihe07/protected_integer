#![feature(test)]

use std::ops::BitXor;

use num::Integer;

pub struct ProtectedInteger<T> {
    value: T,
    verification: T,
    magic: T,
}

#[derive(Debug, Clone, Copy)]
pub enum State<T> {
    Tampered(T),   // value is tampered, but can be restored
    Untampered(T), // value is untampered
}

impl<T: Integer + Copy + BitXor<Output = T>> ProtectedInteger<T>
where
    rand::distributions::Standard: rand::prelude::Distribution<T>,
{
    pub fn new(value: T) -> ProtectedInteger<T> {
        let magic: T = rand::random::<T>();
        let verification = value ^ magic;
        ProtectedInteger {
            value: value.clone(),
            verification: verification.into(),
            magic,
        }
    }
    pub fn get(&self) -> State<T> {
        let value = self.value.clone();
        let verification = self.verification ^ self.magic;
        if verification == value {
            State::Untampered(value)
        } else {
            State::Tampered(verification.into())
        }
    }
    pub fn set(&mut self, value: T) {
        let magic: T = rand::random::<T>();
        let verification = value ^ magic;
        self.value = value;
        self.verification = verification;
        self.magic = magic;
    }
}

#[cfg(test)]
mod tests {
    
    use crate::ProtectedInteger;
    extern crate test;

    #[test]
    fn basic_mutations() {
        let mut number = ProtectedInteger::new(114514);
        dbg!(number.get());
        number.set(1919810);
        dbg!(number.get());
    }
    #[bench]
    fn bench_setting(b: &mut test::Bencher) {
        let mut number = ProtectedInteger::new(114514);
        b.iter(|| {
            number.set(test::black_box(1919810));
        });
    }
    #[bench]
    fn bench_getting(b: &mut test::Bencher) {
        let number = ProtectedInteger::new(114514);
        b.iter(|| {
            test::black_box(number.get());
        });
    }
}
