use std::io;

fn main() {
    loop {
        println!("Enter an operation (add, subtract, multiply, divide) or 'quit' to exit:");
        
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        let operation = operation.trim();
        
        if operation == "quit" {
            break;
        }
        
        println!("Enter the first number:");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = num1.trim().parse().expect("Invalid number");
        
        println!("Enter the second number:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = num2.trim().parse().expect("Invalid number");
        
        let result = match operation {
            "add" => num1 + num2,
            "subtract" => num1 - num2,
            "multiply" => num1 * num2,
            "divide" => num1 / num2,
            _ => {
                println!("Invalid operation");
                continue;
            }
        };
        
        println!("Result: {}", result);
    }
}
