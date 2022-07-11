use std::num::ParseIntError;

#[warn(unused_variables)]
fn main() {
    // how to implement the 'parse' method? using marco for i32 i64 ...
    let x = "bb".parse::<i32>();
    // println!("{}", x.unwrap());
    // TODO error
    // try!  ?
    let result1 = parse("bb");
    // print(result1);
    println!("? x is {}", result1.unwrap_err());

    let result = multiply("a", "1");
    print(result);
}

fn parse(str: &str) -> Result<i32, ParseIntError> {
    let x = str.parse::<i32>()?;
    Ok(x)
}


fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}


fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}