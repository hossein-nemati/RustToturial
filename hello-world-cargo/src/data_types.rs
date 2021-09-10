pub fn data_types() {
    println!("data_types function");

    // set mut flag to change variable value
    let mut x = 40;

    println!("The value of x is {}", x);
    x = 60;
    println!("The value of x is {}", x);

    let _a = 40; // i32
    let _b: u32 = 50; // u32
    let _c: bool = false;
    let _d = 'z';
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    //   Tuple

    let (_x, _y, _z) = _tup;

    println!("The value of y is: {}", _y);

    let _five_hundred = _tup.0;

    let _six_point_four = _tup.1;

    let _one = _tup.2;

    println!("The value of  five_hundred is: {}", _five_hundred);

    let _tup1 = (20, "Rust", 3.4, false, (1, 2, 3));

    println!("The nested tuple value in position is {}", (_tup1.4).2);


    let _tup2 = (1, 2, "Rust");
    let (a, b, c) = _tup2;

    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);


    // Array

    let mut _array:[i32; 5] = [1, 2, 3, 4, 5];
    _array  = [1, 2, 3, 4, 5];

    let _array_b = [3; 5];

    println!("The first value of array is: {}", _array[0]);
    println!("The first value of array_b is: {}", _array_b[0]);




    // Enum


    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green
        // // These likewise tie `u32` tuples to different names: color models.
        // RGB(u32, u32, u32),
        // HSV(u32, u32, u32),
        // HSL(u32, u32, u32),
        // CMY(u32, u32, u32),
        // CMYK(u32, u32, u32, u32)
    }

    let red:Color = Color::Red;

    match red {
        Color::Red => println!("Color is red!"),
        Color::Blue => println!("Color is blue!"),
        _ => println!("Color is ***")
    }



    // Struct

    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    
    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };
    
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),
    
        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
    
        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }
    


    // Const

    const MAXIMUM_NUMBER: u8 = 3;

    for n in 1..MAXIMUM_NUMBER {
        println!("count until maximum number {}", n);
    }



    // Code Blocks & Shadowing

    let mut x = 10;

    {
        let x = 15;
    }
    println!("x is {}", x);
    let x = "X is a string";
    println!("x is {}", x);

    let x = true;
    println!("x is {}", x);
}