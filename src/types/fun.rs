use std::iter::Enumerate;

use super::var::Var;
use crate::util::processing;


#[derive(Debug)]
pub struct Fun {
    pub name: String,
    pub parameters: Vec<Var>,
    pub return_type: String,
    pub code: Vec<String>
}
// todo impl get return type
// todo impl get parameters etc. in new


// todo fixed, but still hideous xd feel free to update
// fn get_parameters_old(mut params: &[String]) -> Vec<Var> {
//     params = &params[1..params.len() - 1];

//     let mut parameters: Vec<Var> = Vec::new();

//     let mut comma_index: Vec<usize> = Vec::new();
//     let mut colon_index: Vec<usize> = Vec::new();
//     let mut equals_index: Vec<usize> = Vec::new();

//     let mut default_value: Vec<String> = vec!["".to_string()];

//     let mut i: usize = 0;
//     for string in params {
//         match string.as_str() {
//             "," => comma_index.push(i),
//             ":" => colon_index.push(i),
//             "=" => equals_index.push(i),
//             _ => {}
//         }

//         i += 1;
//     }

//     i = 0;
//     let mut i_equals: i32 = 0;
//     for colon_idx in &colon_index {
//         if let Some(equals_idx) = equals_index.get(i_equals as usize) {
//             if let Some(comma_idx) = comma_index.get(i) {
//                 if comma_idx < equals_idx {
//                     i_equals -= 1;
//                 } else {
//                     default_value = params[*equals_idx + 1..*comma_idx].join(" ");
//                 }
//             } else {
//                 default_value = params[*equals_idx + 1..];
//             }
//         }

//         parameters.push(
//             Var::new(
//                 params[colon_idx - 1].clone(),
//                 params[colon_idx + 1].clone(),
//                 default_value.clone()
//             ));

//         i_equals += 1;
//         i += 1;
//         default_value = vec!["".to_string()];
//     }

//     parameters
// }


// proper type_fncy -> type_rs conversion needs to be done in Var
pub fn and_another_one(strings: &[String]) -> Vec<Var>{
    let mut marker = 0;
    let mut last_id_char = ",";

    let mut name = "".to_string();
    let mut type_fncy = "".to_string();
    let mut value: Vec<String> = Vec::new();

    let mut var_vec: Vec<Var> = Vec::new();


    for (i, string) in strings.iter().enumerate() {
        match string.as_str() {
            ":" => {
                name = strings[marker+1..i].join("");

                last_id_char = ":";
                marker = i;
            },
            "=" => {
                type_fncy = strings[marker+1..i].join(" ");

                last_id_char = "=";
                marker = i;
            }
            s if s == "," || i == strings.len()-1 => {
                if last_id_char == ":" { type_fncy = strings[marker+1..i].join(" ") }
                else if last_id_char == "=" { value = strings[marker+1..i].to_vec() }

                var_vec.push(Var::new(name, type_fncy, value));

                name = "".to_string();
                type_fncy = "".to_string();
                value = Vec::new();

                last_id_char = ",";
                marker = i;
            }
            _ => {}
        }
    }


    dbg!(&var_vec);

    var_vec
}

// how bout we write a cleaner one xd
// pub fn get_parameters(mut slice: &[String]) -> Vec<Var> {
//     slice = &slice[1..slice.len() - 1];
//     let mut i: usize = 0;
//     let slice_len: usize = slice.len();

//     let mut params: Vec<Var> = Vec::new();
//     let mut expression: Vec<String> = Vec::new();
//     let mut name: String = "".to_string();
//     let mut type_fncy: Vec<String> = Vec::new();

//     if slice.len() == 0 {
//         return params
//     }

//     let mut last_id_string: &str = ",";

//     if let Some(next_colon_pos) = slice.iter().position(|x| x==":") {
//         name = slice[i..next_colon_pos].join("");
//         i = next_colon_pos;
//     }
//     if let Some(next_comma_pos) = slice.iter().position(|x| x==",") {
//         // type_fncy = slice[i..next_comma_pos]
//     }
//     slice.iter().position(|x| x==",");
//     slice.iter().position(|x| x=="=");

//     params
    // slice = &slice[1..slice.len() - 1];
    // let mut i: usize = 0;
    // let slice_len: usize = slice.len();

    // let mut params: Vec<Var> = Vec::new();
    // let mut expression: Vec<String> = Vec::new();
    // let mut name: String = "".to_string();
    // let mut type_fncy: Vec<String> = Vec::new();

    // if slice.len() == 0 {
    //     return params
    // }

    // let mut last_id_string: &str = ",";
    // vec![""].append()

    // // for string in slice.iter().chain([",".to_string()].iter()) {
    // for current_string in slice {
    //     // last_string = match string.as_str() {
    //     match (last_id_string, current_string.as_str(), i) {
    //         x if (x.0 == "," && x.1 != ":" && x.2 != 0) || i == slice_len => { // last entry won't be included...
    //             if expression.is_empty() { expression.push("".to_string()); }
    //             params.push(Var::new(name.clone(), type_fncy.join(" "), expression.clone()));

    //             name = "".to_string();
    //             type_fncy.clear();
    //             expression.clear();
    //         }
    //         x if x.0 == "," && x.1 != ":" => name = current_string.clone(),
    //         (",", ":", _) => name = current_string.clone(),
    //         (":", ",", _) => type_fncy.push(current_string.clone()),
    //         (":", "=", _) => expression.push(current_string.clone()),
    //         _ => {
    //             dbg!(&last_id_string);
    //             dbg!(&current_string);
    //             dbg!(&i);
    //             // otherwise, name, type, or expression
    //         }
    //     }

    //     last_id_string = match current_string.as_str() {
    //         "," => ",",
    //         ":" => ":",
    //         "=" => "=",
    //         _ => {last_id_string}
    //     };

    //     i += 1;
    // }
    // dbg!(&params);

    // params




    // for string in slice {
    //     // last_string = match string.as_str() {
    //     match last_id_string {
    //         "," => {
    //             // next is name
    //             name = string.clone();
    //         },
    //         ":" => {
    //             // next is type
    //             type_fncy.push(string.clone());
    //         }
    //         "=" => {
    //             // next is expression
    //             expression.push(string.clone());
    //         }
    //         _ => {
    //             // otherwise, name, type, or expression
    //         }
    //     }

    //     last_id_string = match string.as_str() {
    //         "," => {
    //             // next is name
    //             if !&name.is_empty() && !&type_fncy.is_empty() {
    //                 if expression.is_empty() { expression.push("".to_string()); }

    //                 params.push(Var::new(name.clone(), type_fncy.join(" "), expression.clone()));
    //             }

    //             name = "".to_string();
    //             type_fncy.clear();
    //             expression.clear();

    //             ","
    //         },
    //         ":" => {
    //             // next is type
    //             ":"
    //         },
    //         "=" => {
    //             // next is expression
    //             "="
    //         }
    //         _ => {
    //             // otherwise, name, type, or expression
    //             last_id_string
    //         }
    //     };
    // }
    // dbg!(&params);

    // params
// }


fn get_fun(slice: &[String]) {

}

fn fun_def_handler(code: &[String]) -> (Fun, usize) {
    if code.first().expect("no fun, nothing at all tbh") != "fun" { panic!("no fun indeed") };

    let fun_name = code[1].clone();

    let end_of_params = processing::get_i_of_next_matching_parenthesis(&code[2..]) + 2;
    let raw_params = &code[2..=end_of_params];
    // let fun_params = get_parameters(raw_params);
    let fun_params = and_another_one(raw_params);

    let start_of_code = end_of_params + code[end_of_params..].iter().position(|s| s == "{").expect("no { after fun_def");
    let raw_return_type = &code[end_of_params+1..start_of_code];
    let return_type = if raw_return_type.is_empty() { "".to_string() } else { raw_return_type[1..].join(" ") };

    let end_of_code = start_of_code + processing::get_i_of_next_matching_brace(&code[start_of_code..]);
    let fun_code = code[start_of_code..=end_of_code].to_vec();

    let my_fun = Fun {
        name: fun_name,
        parameters: fun_params,
        return_type,
        code: fun_code
    };

    (my_fun, end_of_code)
}

fn fun_call_handler(code: &Vec<String>) {

}