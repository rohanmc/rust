// need to add this import to make stdout.flush work ???
use std::io::prelude::*;

use std::io;
fn main() {
    print!("enter temp in celsius: ");
    // doesn't work without use std::io::prelude::* .. complains about missing trait?
    io::stdout().flush().ok().expect("Could not flush stdout");

    let mut inp = String::new();
    io::stdin().read_line(&mut inp).expect("Could not read line");

    //converting input string into number
    let inp_temp : i32 = inp.trim().parse().expect("Not a number");
    let conv = ( inp_temp as f64 ) * 9./5. + 32.;
    println!("Temp in Farenheit: {}",conv);
}
