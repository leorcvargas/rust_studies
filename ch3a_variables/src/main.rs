fn main() {
    // `mut` is necessary here, otherwise it would not compile because of
    // line 3
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");
}
