fn main() {
    let result = factorial(5);
    println!("5!={}", result);
}

fn factorial(number: i32) -> i32 {
    if number == 0 {
        return 1;
    } else {
        return number * factorial(number - 1);
    }
}

// In Rust, the main() function is the entry point for the program, just like in Java. The println! macro is used to print a message to the console, similar to the System.out.println method in Java.

// The factorial() function in Rust is similar to the one in Java, but it uses the if expression to return a value based on a condition, rather than using an if statement. Additionally, the return type of the factorial() function is specified using a -> followed by the return type after the parameter list.

// I hope this helps! Let me know if you have any other questions.