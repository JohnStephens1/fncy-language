// grouping symbols

#[derive(Default)]
pub struct GroupingSymbols {
    pub parentheses: GroupingSymbol,
    pub brackets: GroupingSymbol,
    pub braces: GroupingSymbol,
    pub angle_brackets: GroupingSymbol
}

#[derive(Default)]
pub struct GroupingSymbol {
    pub open: Vec<usize>,
    pub closed: Vec<usize>,
}

#[derive(Default)]
pub struct Idx {
    pub grouping_symbols: GroupingSymbols
}





// impl GroupingSymbol {
//     // just calling open.push(idx) works fine xd
//     pub fn add_open(&mut self, idx: usize) {
//         self.open.push(idx);
//     }

//     pub fn add_closed(&mut self, idx: usize) {
//         self.closed.push(idx);
//     }
// }