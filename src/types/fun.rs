use super::var::{Var, VarInfo};
use crate::util::processing;


// todo impl get return type
// todo impl get parameters etc. in new
#[derive(Debug)]
pub struct Fun {
    pub name: String,
    pub parameters: Vec<Var>,
    pub return_type: ReturnType,
    pub code: Vec<String>
}

// todo
// automate parameter getting
impl Fun {
    pub fn new(name: String, parameters: Vec<Var>, return_type_fncy_raw: String, code: Vec<String>) -> Self {
        let return_type = ReturnType::new(return_type_fncy_raw);

        Fun {
            name,
            parameters,
            return_type,
            code
        }
    }
}

#[derive(Debug)]
pub struct ReturnType {
    pub type_fncy: String,
    pub type_rs: String,
    pub var_info: VarInfo
}

impl ReturnType {
    pub fn new(type_fncy: String) -> Self {
        let (type_fncy, type_rs, var_info) = super::util::process_type_fncy(&type_fncy);

        if var_info.is_var_ref { panic!("return type cannot be of type var_ref: v&\nUse <type> or &v <type> instead") }
        if !var_info.is_ref && var_info.is_var { panic!("return type cannot be explicitly mutable\nUse <type> or &v <type> instead") }

        ReturnType {
            type_fncy,
            type_rs,
            var_info
        }
    }
}


// proper type_fncy -> type_rs conversion needs to be done in Var
pub fn get_parameters(strings: &[String]) -> Vec<Var>{
    let mut marker = 0;
    let mut last_id_char = ",";

    let mut var_vec: Vec<Var> = Vec::new();
    if strings == ["(", ")"] { return var_vec }

    let mut name = "".to_string();
    let mut type_fncy = "".to_string();
    let mut value: Vec<String> = Vec::new();


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


fn get_fun(slice: &[String]) {

}

pub fn fun_def_handler(code: &[String]) -> (Fun, usize) {
    if code.first().expect("no fun, nothing at all tbh") != "fun" { panic!("no fun indeed") };

    let fun_name = code[1].clone();

    let end_of_params = processing::get_i_of_next_matching_parenthesis(&code[2..]) + 2;
    let raw_params = &code[2..=end_of_params];
    let fun_params = get_parameters(raw_params);

    let start_of_code = end_of_params + code[end_of_params..].iter().position(|s| s == "{").expect("no { after fun_def");
    let raw_return_type = &code[end_of_params+1..start_of_code];
    let return_type = if raw_return_type.is_empty() { "".to_string() } else { raw_return_type[1..].join(" ") };

    let end_of_code = start_of_code + processing::get_i_of_next_matching_brace(&code[start_of_code..]);
    let fun_code = code[start_of_code..=end_of_code].to_vec();

    let my_fun = Fun::new(
        fun_name,
        fun_params,
        return_type,
        fun_code
    );

    (my_fun, end_of_code)
}

fn fun_call_handler(code: &Vec<String>) {

}
