//! This is a very simple crate that protects some of your integer variables from being modified by memory tamperers such as Cheat Engine. And check that they have not been attempted to be tampered with.
//! ## Example
//! ```rust
//! use protected_integer::{ProtectedInteger, State};
//!
//! let mut my_number = ProtectedInteger::new(114514); // initialize this variable
//!
//! // Check and get the value
//! match my_number.get() {
//!     State::Untampered(num) => {
//!         println!("The variable has not been tampered with");
//!     }
//!     State::Tampered(num) => {
//!         println!("The variable was tampered with, but the backup variable was not");
//!         println!("The restored value is {}", num);
//!     }
//! }
//!
//! // if you just want to get the value
//! println!("The value is {}", my_number.get().to_value());
//!
//! // change the value
//! my_number.set(1919810);
//! ```
//! For more information, check it on [GitHub](https://github.com/lihe07/protected_integer)

#![feature(test)]

use std::ops::BitXor;

use num::Integer;

/// The protected integer
/// ```rust
/// use protected_integer::ProtectedInteger;
/// let mut my_number = ProtectedInteger::new(114514); // initialize this variable
/// println!("{}", my_number.get().to_value());
/// ```
pub struct ProtectedInteger<T> {
    value: T,
    verification: T,
    magic: T,
}

/// The state of the protected integer
/// ```rust
/// use protected_integer::ProtectedInteger;
/// let some_protected = ProtectedInteger::new(114514);
///
/// let state = some_protected.get();
/// println!("is_tampered: {}", state.is_tampered());
/// println!("is_untampered: {}", state.is_untampered());
/// println!("to_value: {}", state.to_value());
/// ```
#[derive(Debug, Clone, Copy)]
pub enum State<T> {
    Tampered(T),   // value is tampered, but can be restored
    Untampered(T), // value is untampered
}

impl<T> State<T> {
    // Check if the value is tampered
    pub fn is_tampered(&self) -> bool {
        match self {
            State::Tampered(_) => true,
            _ => false,
        }
    }
    /// Check if the value is untampered
    pub fn is_untampered(&self) -> bool {
        match self {
            State::Untampered(_) => true,
            _ => false,
        }
    }
    /// Converts the state to the value
    /// WARN: This will ignore whether the value is tampered or not, and consume the state
    pub fn to_value(self) -> T {
        match self {
            State::Tampered(num) => num,
            State::Untampered(num) => num,
        }
    }
}

impl<T: Integer + Copy + BitXor<Output = T>> ProtectedInteger<T>
where
    rand::distributions::Standard: rand::prelude::Distribution<T>,
{
    /// Create a new protected integer
    /// A random value will be generated for the verification key
    pub fn new(value: T) -> ProtectedInteger<T> {
        let magic: T = rand::random::<T>();
        let verification = value ^ magic;
        ProtectedInteger {
            value: value.clone(),
            verification: verification.into(),
            magic,
        }
    }
    /// Get the value of the protected integer and check if it is tampered
    pub fn get(&self) -> State<T> {
        let value = self.value.clone();
        let verification = self.verification ^ self.magic;
        if verification == value {
            State::Untampered(value)
        } else {
            State::Tampered(verification.into())
        }
    }
    /// Set the value of the protected integer, and update the verification key
    pub fn set(&mut self, value: T) {
        let magic: T = rand::random::<T>();
        let verification = value ^ magic;
        self.value = value;
        self.verification = verification;
        self.magic = magic;
    }
    /// Check if the value is not tampered
    pub fn check(&self) -> bool {
        let verification = self.value ^ self.magic;
        if verification == self.verification {
            true
        } else {
            false
        }
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
        dbg!(number.check());
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
