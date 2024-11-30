use std::env::{args, Args};

fn main(){
    let mut args: Args = args();

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap();
    let second = args.nth(0).unwrap();
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result = operate( &operator,first_number, second_number);

    println!("{}",readability(first_number, second_number, &operator, result));

}

fn operate(operator: &str,first_number: f32, second_number: f32) -> f32{
    match operator{
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" | "x" | "X" => first_number * second_number,
        "/" => first_number / second_number,
        "%" => first_number % second_number,
        _=> panic!("BAD OPERATOR"),
    }
}

fn readability(first_number: f32, second_number: f32, operator: &str, result: f32) -> String{
    format!("{first_number} {operator} {second_number} = {result}")
}
