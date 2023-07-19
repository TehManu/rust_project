const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn variables_let() {
    let x = 5;
    println!("The value of x is: {x}");
    //x = 6; <- Doesn't compile because variable is not mutable!
    println!("The value of x is: {x}");
}

pub fn variables_mut() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    print!("The value of x is: {x}");
}

pub fn variables_const() {
    println!("{THREE_HOURS_IN_SECONDS}");
}

pub fn variables_shadowing() {
    let x = 5;

    // Create a new variable x with the original value of x and add + 1
    let x = x + 1;

    {
        let x = x * 5;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

pub fn variables_shadowing_2() {
    let spaces = "   ";
    let spaces = spaces.len();

    print!("{spaces}");

    //let mut spaces = "   ";
    // Doesn't work because len() is usize not string!
    //spaces = spaces.len();
}



