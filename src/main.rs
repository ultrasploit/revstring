use revstring::Reversing;
mod utils;

// ==============================================================================
// | Kindly review the README file thoroughly before examining the source code. |
// ==============================================================================
fn main() {
    let string = utils::io::get_input("Input: ");

    println!("============================================");
    println!("Fully reversed: {}", string.reverse());
    println!("============================================");
    println!("Words reversed: {}", string.reverse_words());
    println!("============================================");
    println!("Sentense palindrome: {}", string.is_palindrome());
    println!("============================================");
    println!("All words palindrome: {}", string.is_words_palindrome());
    println!("============================================");
    println!("Palindrome words: ");
    dbg!(string.get_palindrome_words());
}
