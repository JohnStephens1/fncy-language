use super::util::split_type_fncy_raw;
use super::util::get_type_rs;


#[derive(Debug, PartialEq)]
pub struct Var {
    // add references
    pub name: String,
    pub type_fncy: String,
    pub type_rs: String,
    pub var_info: VarInfo,
    pub value: Vec<String>
}

impl Var {
    pub fn new(name: String, type_fncy_raw: String, value: Vec<String>) -> Self {
        //todo
        //opt translate value
        //later: add formatters for use cases, type_fncy and type_rs

        let (prev, type_fncy) = split_type_fncy_raw(&type_fncy_raw);
        let var_info = VarInfo::new(&prev);
        let type_rs = get_type_rs(&type_fncy);

        Self {
            name,
            type_fncy: type_fncy_raw,
            type_rs,
            var_info,
            value
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct VarInfo {
    pub ref_count: usize,
    pub is_ref: bool,
    pub is_var_ref: bool,
    pub is_var: bool,
}

impl VarInfo {
    pub fn new(type_fncy_prefix: &String) -> Self {
        let ref_count: usize = type_fncy_prefix.matches("&").count();
        let is_ref: bool = if ref_count > 0 { true } else { false };
        let is_var_ref: bool = if type_fncy_prefix.matches("v&").count() > 0 { true } else { false };
        let is_var: bool = if type_fncy_prefix.ends_with("v") { true } else { false };

        Self {
            ref_count,
            is_ref,
            is_var_ref,
            is_var
        }
    }
}
