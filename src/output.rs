use super::*;
use std::fmt::{Display, Formatter};

pub const RESULT_MAX_PRIME: u16 = 4095;
const RESULT_DEF_OK: u16 = 0xA000;
const RESULT_DEF_ERROR: u16 = 0xF000;


/// `Result` enum is a part of a `Output` structure.
///
/// **Ok** and **Error** contain integer representing some status code.
/// Each should be documented in the command help.
///
/// Restricted prime values for **Ok** and **Error** are from `0 to 5`,
/// But can be overwritten.
///
/// If everything was completed successfully without any info,
/// **Ok** should contain prime value `6`, so
/// the status code would be `0xA006`.
///
/// If something has failed without any info,
/// **Error** should contain prime value `6`, so
/// the status code would be `0xF006`.
///
/// 0xF000 - Invalid instruction
/// 0xF001 - Unknown command
pub enum Result {
    Ok(u16),
    Error(u16),
}

impl Result {
    fn parse_status_code(res: &str, def_val: u16, val: u16) -> u16 {
        if val > RESULT_MAX_PRIME {
            panic!("Exceed max value of {} status code!\nMax value is: [{}]\nProvided value was: [{}]", res, RESULT_MAX_PRIME, val)
        }

        def_val + val
    }

    /// Indicate Successful result status code by a prime value
    pub fn ok(prime_val: u16) -> Self {
        Self::Ok(Self::parse_status_code(
            "Ok",
            RESULT_DEF_OK,
            prime_val,
        ))
    }

    /// Indicate Failed result status code by a prime value
    pub fn err(prime_val: u16) -> Self {
        Self::Ok(Self::parse_status_code(
            "Error",
            RESULT_DEF_ERROR,
            prime_val,
        ))
    }

    /// Return the full status code of the Result.
    ///
    /// (prefix + prime value)
    pub fn status_code(&self) -> u16 {
        return match self {
            Result::Ok(val) => *val,
            Result::Error(val) => *val,
        }
    }

    /// Return only prime value of the Result
    pub fn raw_val(&self) -> u16 {
        return match self {
            Result::Ok(val) => *val - RESULT_DEF_OK,
            Result::Error(val) => *val - RESULT_DEF_ERROR,
        }
    }
}

impl Display for Result {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "0x{:X}",
            self.status_code(),
        )
    }
}


/// Undocumented
pub struct Output {
    pub result: Result,
    pub message: String,
}

impl Output {
    fn new(result: Result, msg: Option<String>) -> Self{
        let message = match msg {
            None => "".to_string(),
            Some(message) => message,
        };

        Self {
            result,
            message,
        }
    }

    pub fn new_ok(prime_val: u16, msg: Option<String>) -> Self {
        Self::new(
            Result::ok(prime_val),
            msg,
        )
    }

    pub fn new_error(prime_val: u16, msg: Option<String>) -> Self {
        Self::new(
            Result::err(prime_val),
            msg,
        )
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} - [{}]",
            self.result,
            self.message,
        )
    }
}
