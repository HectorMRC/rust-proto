use crate::transaction::traits::{Body};

use std::error::Error;
use std::any::Any;

pub struct TxRequest<'a> {
    msg: &'a str,
}

impl<'a> TxRequest<'a> {
    pub fn new(name: &'a str) -> Self {
        TxRequest{
            msg: name,
        }
    }
}

impl<'a> Body for TxRequest<'a> {
    fn precondition(&self) -> Result<(), String> {
        Ok(())
    }

	fn postcondition(&self) -> Result<Box<dyn Any>, String> {
        Ok(Box::new(String::new()))
    }

	fn commit(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	fn rollback(&self) {

    }

}