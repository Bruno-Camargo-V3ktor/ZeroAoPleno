use std::ops::Div;

#[derive(Debug, PartialEq)]
pub enum DivisionError {
    DivisionByZero,
    InvalidOperation,
    OutOfRange,
}

pub fn divide<T>(dividend: T, divisor: T) -> Result<T, DivisionError>
where
    T: Div<Output = T> + PartialEq + From<u8> + Copy + PartialOrd,
{
    if divisor == T::from(0 as u8) {
        return Err(DivisionError::DivisionByZero);
    }

    let res = dividend / divisor;
    if res == T::from(0 as u8) {
        return Err(DivisionError::InvalidOperation);
    }

    if res > T::from(10 as u8) {
        return Err(DivisionError::OutOfRange);
    }

    Ok(res)
}
