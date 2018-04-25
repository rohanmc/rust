fn main() {
    let x = String::from("rohan");

    let y = &&&&x;
    println!("{:?}",(x,y));
}
