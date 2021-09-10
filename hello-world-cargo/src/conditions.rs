pub fn conditions()
{
    println!("conditions function");

    let number = 30;

    if number < 30 {
        println!("if condition passed!");
    } else {
        println!("else condition passed!");
    }

    match number {
        30 => println!("match first condition passed!"),
        13..=19 => println!("A teen"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        _ => println!("Ain't special")
    }
}


