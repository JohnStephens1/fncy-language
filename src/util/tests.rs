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

    assert_eq!(le_match, test_expected_match);
    assert_eq!(remainder, test_expected_remainder);
}
