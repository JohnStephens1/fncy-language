#[cfg(test)]
use pretty_assertions::assert_eq;

use super::*;


#[test]
fn test_split_at_next_delim() {
    let test: Vec<String> = 
    ["\"", "hello", "chikn", "mc", "nuggit", "\"", "hello", "two", "\"", "yuhs", "\"", "eh"]
        .into_iter().map(String::from).collect();

    let test_expected_match: Vec<String> = ["\"", "hello", "chikn", "mc", "nuggit", "\""]
        .into_iter().map(String::from).collect();

    let test_expected_remainder: Vec<String> = ["hello", "two", "\"", "yuhs", "\"", "eh"]
        .into_iter().map(String::from).collect();

    let (le_match, remainder) = processing::split_at_next_delim(&test, "\"");
    assert_eq!(&test_expected_match, &le_match);
    assert_eq!(&test_expected_remainder, &remainder);
}

#[test]
fn test_split_matching_brace() {
    let test_vec: Vec<String> = "{ hello there { } potato potato ( ( ( ) ) ) ( hello ) { } } helooooooo mah nehme heff { } ( ) ) ) { { ("
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let test_expected_match: Vec<String> = "{ hello there { } potato potato ( ( ( ) ) ) ( hello ) { } }"
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let test_expected_remainder: Vec<String> = "helooooooo mah nehme heff { } ( ) ) ) { { ("
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let (le_match, remainder): (Vec<String>, Vec<String>) = processing::split_matching_brace(&test_vec);
    assert_eq!(&test_expected_match, &le_match);
    assert_eq!(&test_expected_remainder, &remainder);
}
