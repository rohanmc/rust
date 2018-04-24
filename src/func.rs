fn main() {
    let x:i32 = 10;
    let y:i32 = 20;
    println!("{} + {} = {}",x,y,add(x,y));
    println!("{} + {} = {}",x,y,add2(x,y));
    println!("{:?}",exprtest());
//    println!("{:?}",exprtest2());
}

fn add(x: i32, y: i32) -> i32 {
    return x+y
}

fn add2(x: i32, y: i32) -> i32 {
    // expressions return values
    // last expression is returned by a function
    // x+y below does not need an explicit 'return'`:wq
    x+y
}

fn exprtest() {
    let x= 1;
}


// the below doesn't work
// because let is a statement and not an expression
// let cannot end without a semicolon
//fn exprtest2() {
//    let x= 1
//}
