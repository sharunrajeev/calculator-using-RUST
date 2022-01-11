fn calculate(num1: i32, num2: i32, op: char) -> i32 {
    match op {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => 0,
    }
}

fn main() {
    println!("--- RUST Calculator ---");
    let first_num: i32;
    let second_num: i32;
    let operation: char;
    println!("Enter nummerical expression in the format a + b = : ");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input_vec: Vec<&str> = input.split(" ").collect();
            first_num = input_vec[0].parse::<i32>().unwrap();
            operation = input_vec[1].to_string().chars().nth(0).unwrap();
            second_num = input_vec[2].parse::<i32>().unwrap();
            println!("{} {} {} = {}", first_num, operation, second_num, calculate(first_num, second_num, operation));
        },
        Err(error) => println!("Error: {}", error),
    }
}
