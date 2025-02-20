// can consider changing output to slice as well
pub fn get_i_of_next_matching_char(slice: &[String], start_char: &str, end_char: &str) -> usize{
    let mut match_char_count: i32 = 0;
    let mut i: usize = 0;

    let start_idx: usize;
    let end_idx: usize;

    for string in slice {
        match_char_count += if string == start_char {1} else if string == end_char {-1} else {0};
        i += 1;

        if match_char_count == 0 { break; }
    }

    i
}

pub fn split_matching_char(slice: &[String], start_char: &str, end_char: &str) -> (Vec<String>, Vec<String>) {
    let i = get_i_of_next_matching_char(slice, start_char, end_char);

    let le_match = slice[0..i].to_vec();
    let remainder = slice[i..].to_vec();

    (le_match, remainder)
}

pub fn split_matching_brace(slice: &[String]) -> (Vec<String>, Vec<String>) {
    split_matching_char(slice, "{", "}")
}

pub fn split_matching_parenthesis(slice: &[String]) -> (Vec<String>, Vec<String>) {
    split_matching_char(slice, "(", ")")
}

pub fn split_matching_bracket(slice: &[String]) -> (Vec<String>, Vec<String>) {
    split_matching_char(slice, "[", "]")
}


pub fn split_at_next_delim(slice: &[String], char: &str) -> (Vec<String>, Vec<String>) {
    match slice.get(0) {
        Some(s) => if s != char { println!("first char wasn't {}!!", &char) },
        _ => println!("slice is empty!!")
    }

    let i = match slice[1..].iter().position(|s| s==char) {
        Some(i) => i+2,
        _ => 0
    };


    let le_match = slice[0..i].to_vec();
    let remainder = slice[i..].to_vec();

    (le_match, remainder)
}

// don't work cuz closing boi == opening boi
pub fn split_matching_quote(slice: &[String]) -> (Vec<String>, Vec<String>) {
    split_matching_char(slice, "\"", "\"")
}
