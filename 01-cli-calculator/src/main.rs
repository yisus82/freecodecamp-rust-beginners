use std::env::args;

fn main() {
    let args = args().collect::<Vec<_>>();
    let operand1 = args.get(1).expect("No operand1");
    let operator = args.get(2).expect("No operator");
    let operand2 = args.get(3).expect("No operand2");
    let first_number = operand1.parse::<f32>().unwrap();
    let second_number = operand2.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);
    println!("{} {} {} = {}", operand1, operator, operand2, result);
}

fn operate(operator: &str, operand1: f32, operand2: f32) -> f32 {
    match operator {
        "+" => operand1 + operand2,
        "-" => operand1 - operand2,
        "*" | "x" | "X" => operand1 * operand2,
        "/" => operand1 / operand2,
        _ => panic!("Unknown operator"),
    }
}
