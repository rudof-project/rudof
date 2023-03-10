use std::rc::Rc;
use std::cell::RefCell;

use antlr_rust::{tree::{ParseTreeVisitor}, recognizer::Recognizer, token_factory::TokenFactory, errors::ANTLRError};
use antlr_rust::error_listener::ErrorListener;

#[derive(Debug, Clone)]
pub struct ShExDocErrorListener {
    num_errors: Rc<RefCell<u32>>
}

impl ShExDocErrorListener {
    pub fn new(counter: Rc<RefCell<u32>>) -> ShExDocErrorListener {
        ShExDocErrorListener { num_errors: counter }
    }

    pub fn num_errors(&self) -> u32 {
        *self.num_errors.borrow()
    }
}

impl <'a, T: Recognizer<'a>> ErrorListener<'a, T> for ShExDocErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _e: Option<&ANTLRError>,
    ) {
        println!("Error found...{:?}", self.num_errors.borrow());
        // let mut ne = self.num_errors.borrow_mut();
        *self.num_errors.borrow_mut() += 1;
        println!("Errors updated...{:?}", self.num_errors.borrow());
        eprintln!(
            "line {}:{} Syntax error {} near '{}'",
            line,
            column,
            self.num_errors.borrow(),
            msg
        );
    }
}
