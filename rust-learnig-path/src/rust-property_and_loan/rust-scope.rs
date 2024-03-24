
fn main() {  
    // `mascot` is not valid and cannot be used here, because it's not yet declared.
    {
        let mascot = String::from("ferris");   // `mascot` is valid from this point forward.
        // do stuff with `mascot`.
    }
    // this scope is now over, so `mascot` is no longer valid and cannot be used.
    println!("{}", mascot);
}
