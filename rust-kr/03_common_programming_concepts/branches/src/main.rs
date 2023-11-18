fn main() {
    verify_condition();

    run_if_expression_example();
}

fn verify_condition() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn run_if_expression_example() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}