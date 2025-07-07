//use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    ParseError { message: String },
    //#[snafu(context(false))]
    //AnyhowError {
    //    source: anyhow::Error,
    //    backtrace: Backtrace,
    //},
}

impl Error {
    pub fn new_parse_error(message: &str) -> Self {
        Error::ParseError {
            message: message.to_string(),
        }
    }
}
