# Crate: protected_integer

A simple crate, that protects some variables from being modified by memory tampering tools.

## Usage

1. Add this crate to dependencies in Cargo.toml

   ```toml
   [dependencies]
   protected_integer = "0.1"
   ```

2. Use this crate in your project

   ```rust
   use protected_integer::{ProtectedInteger, State};
   
   let mut my_number = ProtectedInteger::new(114514); // initialize this variable
   
   // Check and get the value
   match my_number.get() {
       State::Untampered(num) => {
           println!("The variable has not been tampered with");
       }
       State::Tampered(num) => {
           println!("The variable was tampered with, but the backup variable was not");
           println!("The restored value is {}", num);
       }
   }
   
   // change the value
   my_number.set(1919810);
   ```

## Performance

According to benchmark result, this crate is **almost** zero-cost

Detailed result:

```
running 3 tests
test tests::basic_mutations ... ignored
test tests::bench_getting ... bench:           0 ns/iter (+/- 0)
test tests::bench_setting ... bench:           4 ns/iter (+/- 0)
```

## Hacking test

You can clone this repo and execute `cargo run --example hacking_test`

Now trying to hack the process with Cheat Engine

