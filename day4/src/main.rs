// main func is used to start all the program
fn main() {
    println!("Hello, world!");
    my_funtion1();

    let num: i32 = 5;
    my_func2(num, true);
    my_func2(10, true);
}

// snake case is used for rust
//another func can be made above main func rust only runs main func and funcs called in it

fn my_funtion1() {
    println!("hello baby g")
}

//parameter in rust

fn my_func2(x: i32, y: bool) {
    println!("your number is {x} {y}")
}
