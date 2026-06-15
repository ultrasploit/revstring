use revstring::Reversing;

#[test]
fn reverse_basics() {
    assert_eq!("hello".reverse(), "olleh");
    assert_eq!("rust".reverse(), "tsur");

    assert_eq!("hello".reverse_words(), "olleh");
    assert_eq!("rust".reverse_words(), "tsur");
}

#[test]
fn reverse_sentence() {
    assert_eq!("hello world".reverse(), "dlrow olleh");
    assert_eq!("hello world".reverse_words(), "olleh dlrow");
}

#[test]
fn reverse_single_char() {
    assert_eq!("x".reverse(), "x");
    assert_eq!("x".reverse_words(), "x");
}

#[test]
fn reverse_empty() {
    assert_eq!("".reverse(), "");
    assert_eq!("".reverse_words(), "");
}

#[test]
fn reverse_spaces() {
    assert_eq!("a b c".reverse(), "c b a");
    assert_eq!("a b c".reverse_words(), "a b c");

    assert_eq!("  a  b  ".reverse(), "  b  a  ");
    assert_eq!("  a  b  ".reverse_words(), "  a  b  ");
}

#[test]
fn reverse_symbols_and_numbers() {
    assert_eq!("12345".reverse(), "54321");
    assert_eq!("12345".reverse_words(), "54321");

    assert_eq!("a!b@c#".reverse(), "#c@b!a");
    assert_eq!("a!b@c#".reverse_words(), "#c@b!a");
}

#[test]
fn reverse_unicode() {
    assert_eq!("café".reverse(), "éfac");
    assert_eq!("café".reverse_words(), "éfac");

    assert_eq!("héllo".reverse(), "olléh");
    assert_eq!("héllo".reverse_words(), "olléh");
}

#[test]
fn reverse_emojis() {
    assert_eq!("🙂😂🔥".reverse(), "🔥😂🙂");
    assert_eq!("🙂😂🔥".reverse_words(), "🔥😂🙂");
}

#[test]
fn reverse_grapheme_cluster() {
    assert_eq!("👨‍👩‍👧‍👦".reverse(), "👨‍👩‍👧‍👦");
    assert_eq!("👨‍👩‍👧‍👦".reverse_words(), "👨‍👩‍👧‍👦");
}

#[test]
fn reverse_trait_on_string() {
    let s = "hello world".to_string();

    assert_eq!(s.reverse(), "dlrow olleh");
    assert_eq!(s.reverse_words(), "olleh dlrow");
}

#[test]
fn reverse_trait_on_str() {
    let s: &str = "rust lang";

    assert_eq!(s.reverse(), "gnal tsur");
    assert_eq!(s.reverse_words(), "tsur gnal");
}

#[test]
fn reverse_long_sentence() {
    let input = "the quick brown fox jumps over the lazy dog";

    assert_eq!(
        input.reverse(),
        "god yzal eht revo spmuj xof nworb kciuq eht"
    );

    assert_eq!(
        input.reverse_words(),
        "eht kciuq nworb xof spmuj revo eht yzal god"
    );
}

#[test]
fn palindrome_basic_words() {
    assert!("madam".is_palindrome());
    assert!("racecar".is_palindrome());
    assert!("level".is_palindrome());
    assert!("radar".is_palindrome());

    assert!(!"hello".is_palindrome());
    assert!(!"rust".is_palindrome());
}

#[test]
fn palindrome_sentences() {
    assert!("".is_palindrome());
    assert!("a".is_palindrome());

    assert!("noon".is_palindrome());
    assert!(!"hello world".is_palindrome());
}

#[test]
fn words_palindrome_basic() {
    assert!(!"hello world".is_words_palindrome());
    assert!(!"rust lang".is_words_palindrome());
    assert!("a b c".is_words_palindrome());
    assert!("madam".is_words_palindrome());
}

#[test]
fn words_palindrome_negative() {
    assert!(!"hello rust world".is_words_palindrome());
    assert!(!"one two three".is_words_palindrome());
}

#[test]
fn get_palindrome_words_basic() {
    let input = "madam racecar hello level world radar rust";

    let mut result = input.get_palindrome_words();
    result.sort();

    let mut expected = vec![
        "madam".to_string(),
        "racecar".to_string(),
        "level".to_string(),
        "radar".to_string(),
    ];
    expected.sort();

    assert_eq!(result, expected);
}

#[test]
fn get_palindrome_words_none() {
    let input = "hello rust world code";

    assert!(input.get_palindrome_words().is_empty());
}

#[test]
fn get_palindrome_words_edge_cases() {
    let input = " madam  radar  level ";

    let mut result = input.get_palindrome_words();
    result.sort();

    let mut expected = vec![
        "madam".to_string(),
        "radar".to_string(),
        "level".to_string(),
    ];
    expected.sort();

    assert_eq!(result, expected);
}
