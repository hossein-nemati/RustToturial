pub fn loops()
{
    println!("loop function");
    let mut n = 0;

    loop {
        n += 1;

        if n == 1 {
            continue;
        }

        if n > 2 {
            break;
        }

        println!("The value of n is {}", n);
    }

    n = 1;

    while n <= 2 {
        println!("n is {}", n);

        n += 1;
    }

    let numbers = 30..32;

    for i in numbers {
        println!("The number is {}", i);
    }

    let lst_string = vec!["str1", "str2", "srt3"];

    for str in lst_string.iter() {
        println!("The str in list {}", str);
    }

    for (index, a) in lst_string.iter().enumerate() {
        println!("The index is {} and the str is {}", index, a);
    }

}