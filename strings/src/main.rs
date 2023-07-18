mod palindrome;

fn main() {
    println!("+++ Welcome to String Tools with +++");
    println!("  _____    _    _    _____   _______ ");
    println!(" |  __ \\  | |  | |  / ____| |__   __|");
    println!(" | |__) | | |  | | | (___      | |   ");
    println!(" |  _  /  | |  | |  \\___ \\     | |   ");
    println!(" | | \\ \\  | |__| |  ____) |    | |   ");
    println!(" |_|  \\_\\  \\____/  |_____/     |_|   ");

    loop {
        println!("Enter a word to check if it is a palindrome:");
        let mut word = String::new();
        std::io::stdin().read_line(&mut word).expect("Failed to read line");
        let word = word.trim();
        if word == "exit" {
            break;
        }
        if palindrome::is_palindrome(word) {
            println!("{} is a palindrome.", word);
        } else {
            println!("{} is not a palindrome.", word);
        }
    }
}
