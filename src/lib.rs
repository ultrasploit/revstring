use unicode_segmentation::UnicodeSegmentation;

/// Reversing a full string.
/// Ex:
///  input:  "hello world"
///  output: "dlrow olleh"
fn reverse(string: &str) -> String {
    let mut reversed = String::new();

    for g in string.graphemes(true) {
        reversed.insert_str(0, g);
    }

    reversed
}

/// Revering words in a string
/// Ex:
///  input:  "hello world"
///  output: "olleh dlrow"
fn reverse_words(string: &str) -> String {
    let mut reversed = String::new();

    let mut writing_index = 0;
    for g in string.graphemes(true) {
        if g == ' '.to_string() {
            reversed.push_str(g); // Inserts the space to the end of the string.
            writing_index = reversed.len(); // Updates the writing index.
            continue;
        }

        // Inserts the letters/characters from left to right of the string, controlled by the
        // writing index.
        reversed.insert_str(writing_index, g);
    }

    reversed
}

fn is_palindrome(string: &str) -> bool {
    string == reverse(string)
}

fn is_words_palindrome(string: &str) -> bool {
    string == reverse_words(string)
}

/// Returns the palindrome words of a string as a vector.
fn get_palindrome_words(string: &str) -> Vec<String> {
    let mut words = vec!["".to_string()];
    let mut palindrome_words: Vec<String> = Vec::new();

    for c in string.chars() {
        if c.is_whitespace() {
            words.push(String::new());
            continue;
        }

        let index = words.len() - 1;
        words[index].push(c)
    }

    let mut i = 0;
    while i < words.len() {
        let reversed = words[i].reverse();
        if reversed == words[i] && !words[i].is_empty() {
            palindrome_words.push(words[i].clone());
        }
        i += 1;
    }

    palindrome_words
}

/// Trait implemention for the reverse and reverse_words.
/// Usage:
///   - string.reverse() returns the reversed version of the string.
///   - string.reverse_words() reverses the words inside the string and returns it.
///   - string.is_palindrome() returns true if the reverse version of the whole sentense equals the
///     original sentense, else false
///   - string.is_words_palindrome() returns true if the string.reverse_words() equals the original
///     version of the string.
///   - .get_palindrome_words() returns the palindrome words of a string as a vector.
pub trait Reversing {
    fn reverse(&self) -> String;
    fn reverse_words(&self) -> String;
    fn is_palindrome(&self) -> bool;
    fn is_words_palindrome(&self) -> bool;
    fn get_palindrome_words(&self) -> Vec<String>;
}

/// Trait implemention for String.
impl Reversing for String {
    fn reverse(&self) -> String {
        reverse(self)
    }

    fn reverse_words(&self) -> String {
        reverse_words(self)
    }

    fn is_palindrome(&self) -> bool {
        is_palindrome(self)
    }

    fn is_words_palindrome(&self) -> bool {
        is_words_palindrome(self)
    }

    fn get_palindrome_words(&self) -> Vec<String> {
        get_palindrome_words(self)
    }
}

/// Traint implemention for &str
impl Reversing for str {
    fn reverse(&self) -> String {
        reverse(self)
    }

    fn reverse_words(&self) -> String {
        reverse_words(self)
    }

    fn is_palindrome(&self) -> bool {
        is_palindrome(self)
    }

    fn is_words_palindrome(&self) -> bool {
        is_words_palindrome(self)
    }

    fn get_palindrome_words(&self) -> Vec<String> {
        get_palindrome_words(self)
    }
}
