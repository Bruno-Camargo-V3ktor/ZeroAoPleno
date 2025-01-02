use division_zero::{DivisionError, divide};

#[test]
fn test_divide_ok() {
    assert_eq!(divide(10, 2), Ok(5));
}

#[test]
fn test_divide_err_division_by_zero() {
    assert_eq!(divide(10, 0), Err(DivisionError::DivisionByZero));
}

#[test]
fn test_divide_err_invalid_operation() {
    assert_eq!(divide(2, 10), Err(DivisionError::InvalidOperation));
}

#[test]
fn test_divide_err_out_of_range() {
    assert_eq!(divide(100, 2), Err(DivisionError::OutOfRange));
}
