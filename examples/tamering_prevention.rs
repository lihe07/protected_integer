use protected_integer::ProtectedInteger;

fn main() {
    let mut money:ProtectedInteger<i16> = ProtectedInteger::new(0);
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" {
            break;
        }
        match money.get() {
            protected_integer::State::Untampered(num) => {
                money.set(num + 1)
            }
            protected_integer::State::Tampered(num) => {
                println!("检测到内存篡改!");
                money.set(num + 1);
            }
        }
        dbg!(money.get());
    }
}