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


#[test]
fn test_extract_var_info() {
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

#[test]
fn test_return_type_rs_string() {
    let test_results: Vec<String> = [
        "int",
        "&int",
        "&&int",
        "&vint",
        "&&vint"
    ].into_iter().map(
        |x| fun::Fun::new("le_name".to_string(), vec![], x.to_string(), vec![])
        .return_type.type_rs_string,
    ).collect();

    let expected_results: Vec<String> = [
        "i32",
        "&i32",
        "&&i32",
        "&mut i32",
        "&&mut i32"
    ].into_iter().map(String::from).collect();

    assert_eq!(&expected_results, &test_results);
}


// todo
// implement
#[test]
fn test_get_type_name_string_rs() {
    // dbg!(util::get_type_name_rs_string(&"var_name".to_string(), &"i32".to_string(), &var::VarInfo::new(&"v&&v".to_string())));
}

#[test]
fn test_get_type_string_rs() {

}