fn main() {}

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    Notdivisible(NotDivisibleError),
    DividebyZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    divident: i32,
    divisor: i32,
}

pub fn divide(divident: i32, divisor: i32) -> Result<i32, DivisionError> {
    if divisor == 0 {
        Err(DivisionError::DividebyZero)
    } else if divident % divisor != 0 {
        Err(DivisionError::Notdivisible(NotDivisibleError {
            divident,
            divisor,
        }))
    } else {
        Ok(divident / divisor)
    }
}

#[allow(dead_code)]
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers
        .into_iter()
        .map(|n| divide(n, 27).unwrap())
        .collect::<Vec<i32>>();

    Ok(division_results)
}

#[allow(dead_code)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect();

    division_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::Notdivisible(NotDivisibleError {
                divident: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DividebyZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_result() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
