use std::rc::Rc;
use std::cell::RefCell;
use crate::parse_error::*;


use antlr_rust::{tree::{ParseTreeVisitor}, recognizer::Recognizer, token_factory::TokenFactory, errors::ANTLRError};
use antlr_rust::error_listener::ErrorListener;

#[derive(Debug, Clone)]
pub struct ShExDocErrorListener {
    errors: Rc<RefCell<Vec<Box<ParseError>>>>
}


impl ShExDocErrorListener {
    pub fn new(errors: Rc<RefCell<Vec<Box<ParseError>>>>) -> ShExDocErrorListener {
        ShExDocErrorListener { errors: errors }
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
        println!("Error found...{:?}", self.errors.borrow());
        let pe = ParseError::new(line, column, msg);
        // let mut ne = self.num_errors.borrow_mut();
        // let mut x = *self.errors.borrow_mut();
        // x.push(pe);
        // println!("Errors updated...{:?}", x);
        eprintln!(
            "line {}:{} Syntax error {} near '{}'",
            line,
            column,
            self.errors.borrow().len(),
            msg
        );
    }
}
