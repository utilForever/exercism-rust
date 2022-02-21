#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;

    let mut stack: Vec<i32> = Vec::new();
    
    for input in inputs {
        match input {
            Add => {
                if stack.len() < 2 {
                    return None;
                }

                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num2 + num1);
            },
            Subtract => {
                if stack.len() < 2 {
                    return None;
                }

                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num2 - num1);
            },
            Multiply => {
                if stack.len() < 2 {
                    return None;
                }

                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num2 * num1);
            },
            Divide => {
                if stack.len() < 2 {
                    return None;
                }

                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();
                stack.push(num2 / num1);
            },
            Value(val) => {
                stack.push(*val);
            },
        }
    }

    if stack.len() != 1 {
        None
    } else {
        Some(*stack.last().unwrap())
    }
}
