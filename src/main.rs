#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::io;//library import

struct Data{
    num1: i32,
    num2: i32,
}

impl Data {
    fn init(&mut self) {
        self.num1 = 0;
        self.num2 = 0;
    }
    fn input(&mut self) -> i32{
        //stdin is a standard input stream use read_line and expect to error
        let mut num = String::new();
        io::stdin()
        .read_line(&mut num)
        .expect("Read Error");

        num = num.trim_end().to_string();

        //check each character in the string to see if it is a valid digit
        for c in num.chars() {
            if !c.is_digit(10) {
                println!("invalid digit found in string");
                return 0;
            }
        }
    
        let a = match num.parse::<i32>() {
            Ok(n) => n,
            Err(_)=> 0,
        };
        a
    }

    fn datos(&mut self){
        self.num1 = self.input();
        self.num2 = self.input();
    }
}
fn main() {
    let mut flag: bool = true;

    let mut calculadora: Data = Data {
        num1: 0,
        num2: 0,
    };

    while flag{
        let mut option_date :i32= 0;
        println!("*** WELCOME ***");
        println!("SELECT AN OPTION:");
        println!("1. Sum");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Go out");

        option_date = calculadora.input();
        calculadora.init();

        match option_date {
            1 =>{
                println!("Sum");
                calculadora.datos();
                println!("{} + {} = {}",calculadora.num1,calculadora.num2, (calculadora.num1+calculadora.num2))
            },
            2 =>{
                println!("Subtraction");
                calculadora.datos();
                println!("{} - {} = {}",calculadora.num1,calculadora.num2, (calculadora.num1-calculadora.num2))
            },
            3 => {
                println!("Multiplication");
                calculadora.datos();
                println!("{} * {} = {}",calculadora.num1,calculadora.num2, (calculadora.num1*calculadora.num2))
            },
            4 =>{
                println!("Division");
                calculadora.datos();
                println!("{} / {} = {}",calculadora.num1,calculadora.num2, (calculadora.num1/calculadora.num2))
            },
            5 => flag = false,
            _ => println!("INCORRECT OPTION!"),
        }
    }
    println!("COMING OUT....");

}

