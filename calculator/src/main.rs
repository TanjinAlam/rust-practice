
use std::{env::{args, Args}};

fn main() {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();


    let first_value : f32 = first.parse::<f32>().unwrap();
    let second_value : f32 = second.parse::<f32>().unwrap();
    let result: f32 = operate(operator, first_value, second_value);
    println!("{:?}", output(first_value, operator, second_value, result));
}


fn operate(operator: char, first_value: f32, second_value: f32) -> f32{
    // if operator == '+' {
    //     first_value + second_value
    // } else if operator == '-' {
    //     first_value - second_value
    // } else if operator == '/' {
    //     first_value / second_value
    // } else if operator == '*' {
    //     first_value * second_value
    // } else {
    //  0.0
    // }

    match operator {
        '+' => first_value + second_value,
        '-' => first_value - second_value,
        '*' | 'x' | 'X' => first_value * second_value,
        '/' => first_value / second_value,
        _ => panic!("invalid operator used.")
    }
}

fn output(first_value: f32, operator: char, second_value: f32, result: f32) -> String {
    format!("{} {} {} = {}",first_value, operator, second_value , result)
}


