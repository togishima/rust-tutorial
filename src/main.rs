fn main() {
    let str_slice = "World";
    println!("Hello, {}", String::from(str_slice));

    for i in 1..16 {
        if i % 15 == 0 {
            println!("FizzBuss");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{}", i);
        }
    }
}
