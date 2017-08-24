use consts::*;
use assayer::Error as AssayerError;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    Validation(AssayerError),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Validation(_) => MSG_ERR_VALIDATION,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Validation(ref inner_err) => Some(inner_err),
        }
    }
}

#[allow(unused_variables)]
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Validation(ref msg) => f.write_str(&err_msg(MSG_ERR_VALIDATION, self.description())),
        }
    }
}

fn err_msg(err_name: &str, err_detail: &str)-> String {
    format!("{}: {}!", err_name, err_detail)
}

impl From<AssayerError> for Error {
    fn from(error: AssayerError) -> Self { Error::Validation(error) }
}
