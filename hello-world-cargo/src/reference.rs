pub fn reference() {

    println!("Reference function");

    // Reference is a different way to refer to an object or data
    let mut _x = 10;
    //let _xr = &_x;
    
    {
        let _dom = &mut _x;
        *_dom += 1;
    }

    println!("x is {}", _x);
}