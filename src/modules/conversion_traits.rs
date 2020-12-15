use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

pub fn show_conversion_traits() {
    println!("From and Into");
    show_from();

    println!("\nTryFrom and TryInto");
    show_try_from();

    println!("\nString conversions");
    show_strings();
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn show_from() {
    let num = Number::from(30);
    println!("Number::from(30) is {:?}", num);

    // Try removing the type declaration
    let num: Number = 5.into();
    println!("let num: Number = 5.into() is {:?}", num);
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn show_try_from() {
    println!("EvenNumber::try_from(8) is {:?}", EvenNumber::try_from(8));
    println!("EvenNumber::try_from(5) is {:?}", EvenNumber::try_from(5));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    println!(
        "let result: Result<EvenNumber, ()> = 8i32.try_into(); is {:?}",
        result
    );
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    println!(
        "let result: Result<EvenNumber, ()> = 5i32.try_into(); is {:?}",
        result
    );
}

use std::fmt;

struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn show_strings() {
    let circle = Circle { radius: 6 };
    println!(
        "Circle to string using fmt::Display: {}",
        circle.to_string()
    );

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum '5' + '10' using the 'parse' function: {:?}", sum);
}
