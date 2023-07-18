// Function to check if a given string is a palindrome
fn is_palindrome(text: &str) -> bool {
    // Reverse the characters in the string and collect them into a new string
    let reversed = text.chars().rev().collect::<String>();
    
    // Compare the original string with the reversed string
    // If they are the same, the text is a palindrome
    text == reversed
}

#[cfg(test)]
mod tests {
    use super::*; // Import the functions from the parent module

    #[test]
    fn test_is_palindrome() {
            // Define a word to test
            let word = "radar";
    
            // Check if the word is a palindrome using the is_palindrome function
            if is_palindrome(word) {
                println!("{} is a palindrome.", word);
            } else {
                println!("{} is not a palindrome.", word);
            }
    }

}
fn main() {
}