fn process(input: u32) {}
fn processString(s: String) {}

fn caller() {
    /*
    Los tipos primitivos se pasan como una copia.
     */
    let n = 1u32;
    process(n); // Ownership of the number in `n` copied into `process`
    process(n); // `n` can be used again because it wasn't moved, it was copied.

    /*
    Las siguientes lineas fallarán. s, una vez pasada a process, no puede utilizarse
     */
    let s = String::from("Hello, world!");
    processString(s); // Ownership of the string in `s` moved into `process`
    processString(s); // Error! ownership already moved.

    /*
    Para tipos complejos, hay que usar .clone()
    */
    let t = String::from("Hello, world!");
    processString(s.clone()); // Passing another value, cloned from `s`.
    processString(s); // s was never moved and so it can still be used.
}
