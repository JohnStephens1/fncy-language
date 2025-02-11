// grouping symbols
struct GroupingSymbols {
    parentheses: GeneralThingy,
    brackets: GeneralThingy,
    braces: GeneralThingy,
    angle_brackets: GeneralThingy
}

struct GeneralThingy {
    open: Vec<usize>,
    closed: Vec<usize>,
}

impl GeneralThingy {
    fn add_open() {}
}

struct Parentheses {
    open: Vec<usize>,
    closed: Vec<usize>
}

struct Brackets {
    open: Vec<usize>,
    closed: Vec<usize>
}

struct Braces {
    open: Vec<usize>,
    closed: Vec<usize>
}

struct AngleBrackets {
    open: Vec<usize>,
    closed: Vec<usize>
}



struct CharacterIdx {

}



pub fn main() {
    // 
}