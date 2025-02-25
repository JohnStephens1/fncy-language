#[cfg(test)]
use pretty_assertions::assert_eq;
use super::*;

#[test]
fn test_split_type_fncy_raw() {
    let test_strings: Vec<&str> = vec![
        "v&&&vhello",
        "&&&vhello",
        "&&vhello",
        "&vhello",
        "vhello",
        "v&vhello",
        "&&&vvhello",
        "v&vvvvvv",
        "vvvvv",
        "hellou",
        "Vhello",
        "v&Vhello",
        "v&& vpataht",
        "&&& hello",
        "&&hello",
        "&hello",
    ];

    let results_expected: Vec<Vec<String>> = [
        ["v&&&v", "hello"],
        ["&&&v", "hello"],
        ["&&v", "hello"],
        ["&v", "hello"],
        ["v", "hello"],
        ["v&v", "hello"],
        ["&&&v", "vhello"],
        ["v&v", "vvvvv"],
        ["v", "vvvv"],
        ["", "hellou"],
        ["", "Vhello"],
        ["v&", "Vhello"],
        ["v&&v", "pataht"],
        ["&&&", "hello"],
        ["&&", "hello"],
        ["&", "hello"]
    ].into_iter().map(|x| x.into_iter().map(String::from).collect()).collect();

    let results: Vec<Vec<String>> = test_strings.iter().map(|x| util::split_type_fncy_raw(x))
    .map(|x| vec![x.0, x.1]).collect();

    assert_eq!(&results_expected, &results);
}


// #[test]
pub fn test_extract_var_info() {
    use var::VarInfo;

    let test_strings: Vec<&str> = vec![
        "",
        "v",
        "&",
        "v&",
        "&v",
        "v&v",
        "v&&&v",
        "&&&v",
        "v&&&"
    ];

    let expected_results: Vec<VarInfo> = vec![
        VarInfo { ref_count: 0, is_ref: false, is_var_ref: false, is_var: false },
        VarInfo { ref_count: 0, is_ref: false, is_var_ref: false, is_var: true },
        VarInfo { ref_count: 1, is_ref: true, is_var_ref: false, is_var: false },
        VarInfo { ref_count: 1, is_ref: true, is_var_ref: true, is_var: false },
        VarInfo { ref_count: 1, is_ref: true, is_var_ref: false, is_var: true },
        VarInfo { ref_count: 1, is_ref: true, is_var_ref: true, is_var: true },
        VarInfo { ref_count: 3, is_ref: true, is_var_ref: true, is_var: true },
        VarInfo { ref_count: 3, is_ref: true, is_var_ref: false, is_var: true },
        VarInfo { ref_count: 3, is_ref: true, is_var_ref: true, is_var: false }
    ];

    let results: Vec<VarInfo> = test_strings.iter().map(|x| var::VarInfo::new(&x.to_string())).collect();

    assert_eq!(&expected_results, &results);
}