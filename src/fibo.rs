use std::io::prelude::*;
use std::io;

fn fibo_n(n: i32) -> i64 {
    let mut a = 0;
    let mut b = 1;
    let mut sum = 0;
    for _ in 1..(n-1) {
        sum = a + b;
        a = b;
        b = sum;
    }
    sum
}

fn main(){
    print!("enter n for nth fibo: ");
    let mut nstr = String::new();
    io::stdout().flush().ok().expect("Could not flush stdout");

    io::stdin().read_line(&mut nstr).expect("Could not read line");

    let n: i32 = nstr.trim().parse().expect("Not a number");
    println!("{}", fibo_n(n));
}
