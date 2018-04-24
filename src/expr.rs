fn main() {
    let _x = 10;
    let y = {
        let z = 200;
        z+20
    };
    if y < 100 {
        println!("y is less than 100");
    } else if y < 200 {
        println!("y is less than 200");
    } else {
        println!("y is not less than 200");
    }

    let somenum = if y < 100 {
        200
    } else {
        300
    };
    println!("value of somenume is {}", somenum)
}
