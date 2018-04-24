fn main() {
    let x:i32 = 10;
    let y:i32 = 20;
    println!("{} + {} = {}",x,y,add(x,y));
}

fn add(x: i32, y: i32) -> i32 {
    return x+y
}
