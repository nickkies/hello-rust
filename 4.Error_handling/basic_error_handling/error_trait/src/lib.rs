#[warn(dead_code)]
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum CalculationError {
    InvalidOperation(char),
    InvalidOperand(String),
    DividedByZero { divident: i8 },
    Overflow,
}

impl Display for CalculationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::InvalidOperation(op) => {
                format!("{op} is not a valid operation. Allowed: +,-,/,*")
            }
            Self::InvalidOperand(num) => {
                format!("{num} is not a valid integer in range [-128, 127]")
            }
            Self::DividedByZero { divident } => {
                format!("Can not divied by zero. Attempting to divide {divident} by 0")
            }
            Self::Overflow => format!("Overflow while performing the operation"),
        };

        f.write_str(&msg)
    }
}

impl Error for CalculationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

pub fn calculate(num1: &str, num2: &str, operation: char) -> Result<i8, CalculationError> {
    let num1 = num1
        .parse::<i8>()
        .map_err(|_| CalculationError::InvalidOperand(num1.to_string()))?;
    let num2 = num2
        .parse::<i8>()
        .map_err(|_| CalculationError::InvalidOperand(num2.to_string()))?;

    match operation {
        '+' => num1.checked_add(num2).ok_or(CalculationError::Overflow),
        '-' => num1.checked_add(num2).ok_or(CalculationError::Overflow),
        '*' => num1.checked_add(num2).ok_or(CalculationError::Overflow),
        '/' => {
            if num2 == 0 {
                return Err(CalculationError::DividedByZero { divident: num1 });
            }
            num1.checked_div(num2).ok_or(CalculationError::Overflow)
        }
        _ => Err(CalculationError::InvalidOperation(operation)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_operation() {
        let res = calculate("12", "20", '^');

        match res {
            Err(e) => {
                assert_eq!(
                    "^ is not a valid operation. Allowed: +,-,/,*",
                    format!("{e}")
                );
            }
            _ => panic!("Error expected!"),
        }
    }

    #[test]
    fn invalid_operand() {
        let res1 = calculate("aa", "bb", '+');
        let res2 = calculate("45", "4.55", '^');

        match (res1, res2) {
            (Err(e1), Err(e2)) => {
                assert_eq!(
                    "aa is not a valid integer in range [-128, 127]",
                    format!("{e1}")
                );
                assert_eq!(
                    "4.55 is not a valid integer in range [-128, 127]",
                    format!("{e2}")
                )
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn divide_by_zero() {
        let res = calculate("45", "0", '/');

        match res {
            Err(e) => {
                assert_eq!(
                    "Can not divied by zero. Attempting to divide 45 by 0",
                    format!("{e}")
                );
            }
            _ => panic!("Error expected"),
        }
    }

    #[test]
    fn overflow() {
        let res1 = calculate("120", "120", '+');
        let res2 = calculate("-120", "-120", '-');

        match (res1, res2) {
            (Err(e1), Err(e2)) => {
                assert_eq!("Overflow while performing the operation", format!("{e1}"));
                assert_eq!(format!("{e1}"), format!("{e2}"));
            }
            _ => panic!("Error expected"),
        }
    }
}
