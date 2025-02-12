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
    pub grouping_symbols: GroupingSymbols,
    pub symbols: Symbols
}


#[derive(Default)]
pub struct Symbols {
    pub minus: Vec<usize>,
    pub plus: Vec<usize>,
    pub asterisk: Vec<usize>,
    pub slash: Vec<usize>,
    pub backslash: Vec<usize>,

    pub equal: Vec<usize>,

    pub and: Vec<usize>,
    pub or: Vec<usize>,
    pub exclamation_mark: Vec<usize>,
    pub question_mark: Vec<usize>,

    pub tilde: Vec<usize>,


    pub comma: Vec<usize>,
    pub semicolon: Vec<usize>,
    pub dot: Vec<usize>,
    pub colon: Vec<usize>,

    pub apostrophe: Vec<usize>,
    pub quotation_mark: Vec<usize>,

    pub forward_tick: Vec<usize>,
    pub back_tick: Vec<usize>,


    pub dollar: Vec<usize>,
    pub percent: Vec<usize>,
    pub caret: Vec<usize>,
    pub degree: Vec<usize>,

    pub hash: Vec<usize>,
    pub at: Vec<usize>,

    pub underscore: Vec<usize>,
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