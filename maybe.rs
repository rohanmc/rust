enum Binary {
    Zero,
    One
}

enum Maybe<T> {
    Just(T),
    Nothing
}

fn random_function(x:i32) -> Maybe<Binary> {
    if x < 5 {
        Maybe::Just(Binary::Zero)
    } else if x < 10 {
        Maybe::Just(Binary::One)
    } else {
        Maybe::Nothing
    }
}

fn binary_to_print(x:Binary) {
    match x {
        Binary::Zero => println!("Zero"),
        Binary::One => println!("One")
    }
}

fn main() {
    match random_function(6) {
        Maybe::Just(z @ Binary::Zero) => binary_to_print(z), 
        Maybe::Just(o @ Binary::One) => binary_to_print(o),
        // key part here is i cant not match for Nothing AND once i do I can't call binary_to_print
        // on it even if i capture it - because it will result in a compile time error, so I'm
        // forced to handle Nothing in a way that typechecks
        n @ Maybe::Nothing => println!("Nothing"),

    }
}
