#[derive(Debug)]
pub struct Fun {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: String,
    pub code: Vec<String>
}