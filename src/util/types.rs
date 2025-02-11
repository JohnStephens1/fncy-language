// grouping symbols
struct GroupingSymbols {
    parentheses: GroupingSymbol,
    brackets: GroupingSymbol,
    braces: GroupingSymbol,
    angle_brackets: GroupingSymbol
}

struct GroupingSymbol {
    open: Vec<usize>,
    closed: Vec<usize>,
}

impl GroupingSymbol {
    fn add_open(&mut self, idx: usize) {
        self.open.push(idx);
    }

    fn add_closed(&mut self, idx: usize) {
        self.closed.push(idx);
    }

    // fn add(&mut self, open_close: OpenClose, idx: usize) {
    //     self.open_close.push(idx);
    // }
}

struct CharacterIdx {

}



pub fn main() {
    // 
}