use crate::request::Request;
use crate::error::*;

pub struct View {
    callback: Box<dyn Fn(Request) -> Result<()>>
}

impl View {
    pub fn new(callback: Box<dyn Fn(Request) -> Result<()>> ) -> Self {
        Self { callback }
    }
}