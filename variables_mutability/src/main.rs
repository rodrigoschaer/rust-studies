fn mutable_var() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}s");
}

fn shadowing_variables() {
    let x = 5;

    let x = x + 1; // shadows former X

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main() {
    println!("Mutable variables");
    mutable_var();
    println!("");


    println!("Constants");
    constants();
    println!("");

    println!("Shadowing Variables");
    shadowing_variables();
    println!("");
}
