use read_input::prelude::*;
fn main() {
    loop {
        let input: String = input().msg("is_bash > ").get();
        if input == "exit" {
            break;
        }
        println!("{}", input);

    }
}
