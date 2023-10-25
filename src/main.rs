fn main() {
    let str_slice = "World";
    println!("Hello, {}", String::from(str_slice));

    for i in 1..16 {
        println!("{}", to_fizz_buzz_string(i));
    }
}

fn to_fizz_buzz_string(number: i32) -> String {
    if number % 15 == 0 {
        return String::from("FizzBuss");
    } else if number % 5 == 0 {
        return String::from("Buzz");
    } else if number % 3 == 0 {
        return String::from("Fizz");
    } else {
        return format!("{}", number);
    }
}