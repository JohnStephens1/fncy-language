#[derive(Debug, PartialEq)]
pub struct Variable {
    // add references
    pub name: String,
    pub is_mutable: bool,
    pub type_fncy: String,
    pub type_rs: String,
    pub value: Vec<String>
}

#[derive(Debug, PartialEq)]
pub struct VarInfo {
    pub ref_count: usize,
    pub is_ref: bool,
    pub is_var_ref: bool,
    pub is_var: bool,
}

fn get_var_info(prev: &String) -> VarInfo {
    let ref_count: usize = prev.matches("&").count();
    let is_ref: bool = if ref_count > 0 { true } else { false };
    let is_var_ref: bool = if prev.matches("v&").count() > 0 { true } else { false };
    let is_var: bool = if prev.ends_with("v") { true } else { false };

    VarInfo {
        ref_count,
        is_ref,
        is_var_ref,
        is_var
    }
}

pub fn extract_var_info(type_fncy: &String) -> VarInfo {
    let le_map = get_param_type_hashmap();
    let (prev, fncy_type) = split_fncy_type(type_fncy);

    let ref_count: usize = prev.matches("&").count();
    let is_ref: bool = if ref_count > 0 { true } else { false };
    let is_var_ref: bool = if prev.matches("v&").count() > 0 { true } else { false };
    let is_var: bool = if prev.ends_with("v") { true } else { false };

    let type_rs: String = le_map.get(&fncy_type).unwrap_or_else(|| { println!("couldn't map type"); &fncy_type} ).to_string();

    // dbg!((
    //     &ref_count,
    //     &is_ref,
    //     &is_var_ref,
    //     &is_var,
    //     &le_type,
    // ));

    VarInfo {
        ref_count,
        is_ref,
        is_var_ref,
        is_var,
        type_fncy: fncy_type,
    }
}

pub fn get_variable(name: String, type_fncy: String, value: Vec<String>) -> Variable {
    let (is_mutable, type_rs) = translate_type_fncy(&type_fncy);

    Variable {
        name,
        is_mutable,
        type_fncy,
        type_rs,
        value,
    }
}