#[cfg(test)]
pub mod tests {
    use super::super::*;
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
}
