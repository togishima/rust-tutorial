fn main() {
    let person = Person {
        first_name: String::from("Bob"),
        last_name: String::from("Marley"),
    };
    println!("{:?}", person);
    person.greet();
    // hello_world();
    // fizz_buzz();
    // print_tomato();
}

trait Greeter {
    fn greet(&self);
}

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
}

impl Greeter for Person {
    fn greet(&self) {
        println!("Hello, I am {} {}", self.first_name, self.last_name);
    }
}

// fn hello_world() {
//     let str_slice = "World";
//     let x = format!("Hello, {}", String::from(str_slice));
//     let y = x; // 所有権の移動
//     // 所有権はyに移動済みなので下記はエラー
//     // println!("{}", x);
//     println!("{}", y);
// }

// fn print_tomato() {
//     let x = "to";
//     {
//         let y = &x; // 借用
//         let z = format!("{}ma", y); // toma
//         println!("{}", z);
//     }
//     println!("{}", x);
// }

// fn fizz_buzz() {
//     for i in 1..16 {
//         println!("{}", to_fizz_buzz_string(i));
//     }

//     let mut counter = 16;
//     while counter <= 20 {
//         println!("{}", to_fizz_buzz_string(counter));
//         counter += 1;
//     }

//     loop {
//         counter += 1;
//         println!("{}", to_fizz_buzz_string(counter));
//         if counter == 30 {
//             break;
//         }
//     }
// }

// fn to_fizz_buzz_string(number: i32) -> String {
//     if number % 15 == 0 {
//         return String::from("FizzBuss");
//     } else if number % 5 == 0 {
//         return String::from("Buzz");
//     } else if number % 3 == 0 {
//         return String::from("Fizz");
//     } else {
//         return format!("{}", number);
//     }
// }