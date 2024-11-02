
// Structs
pub enum Operation {
    Addition(i32, i32),
    Subtraction(i32, i32),
    Multiplication(i32, i32),
    Division(i32, i32)
}

// Functions
pub fn calculate( operation: Operation ) -> Result<i32, &'static str> {

    match operation {
        Operation::Addition(x, y) => Ok(x + y),

        Operation::Subtraction(x, y) => Ok(x - y),

        Operation::Multiplication(x, y) => Ok(x * y),

        Operation::Division(x, y) => {
            if y == 0 { Err("Cannot divide by zero") }
            else { Ok(x / y) }
        },

        _ => Err("Cannot Operation")
    }

}