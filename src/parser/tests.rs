use super::*;


#[test]
fn test_find_end_of_expression_1() {
    let test_1: Vec<String> = "something . something ( hell hello hello ) . guten_tag shoulda_stopped_here hello ( )"
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let test_1_expected_match: Vec<String> = "something . something ( hell hello hello ) . guten_tag"
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let test_1_expected_remainder: Vec<String> = "shoulda_stopped_here hello ( )"
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let (le_match, remainder) = parser::find_end_of_expression(&test_1);
    assert_eq!(le_match, test_1_expected_match);
    assert_eq!(remainder, test_1_expected_remainder);
}

#[test]
fn test_find_end_of_expression_2() {
    let test_2: Vec<String> = "something something ( hell hello hello ) . guten_tag shoulda_stopped_here hello ( )"
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let test_2_expected_match: Vec<String> = ["something"].into_iter().map(String::from).collect();
    let test_2_expected_remainder: Vec<String> = "something ( hell hello hello ) . guten_tag shoulda_stopped_here hello ( )"
        .split_ascii_whitespace().into_iter().map(String::from).collect();

    let (le_match, remainder) = parser::find_end_of_expression(&test_2);
    assert_eq!(le_match, test_2_expected_match);
    assert_eq!(remainder, test_2_expected_remainder);
}

// todo mut x without & mut x can cause issues in some cases
// figure out a solution
#[test]
fn test_translate_type_fncy_compact() {
    let test_type_fncy: Vec<Vec<String>> = [
        vec!["&", "vusize"],
        vec!["&", "usize"],
        vec!["vint"],
        vec!["float"]
        ].into_iter().map(|x| x.into_iter().map(String::from).collect()).collect();
        
    let results_expected: Vec<String> = ["& mut usize", "& usize", "mut i32", "f32"].into_iter().map(String::from).collect();
    let results: Vec<String> = test_type_fncy.iter().map(types::translate_type_fncy_compact).collect();
    debug_assert_eq!(results, results_expected);
}
