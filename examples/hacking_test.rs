use protected_integer::ProtectedInteger;

fn main() {
    let mut money:ProtectedInteger<i16> = ProtectedInteger::new(123);
    println!("The initial amount of money is {}", money.get().to_value());
    println!("Press Enter to increment it by 1");
    println!("Try searching and modifying this value with Cheat Engine");
    println!("Input 'quit' to exit");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "quit" {
            println!("Exiting...");
            break;
        }
        if !money.check() {
            println!("Hacking detected!");
        }
        money.set(money.get().to_value() + 1);        
        println!("The money is now {}", money.get().to_value());
    }
}