fn main() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;
    
    // by default variables are immutable but here it allows to reassign as the same variable
    // name is  being used which is called shadowing
    // since let is creating a new variable you can technically change the type of the previous defined variable
    println!("The value of x is: {}", x);
}
