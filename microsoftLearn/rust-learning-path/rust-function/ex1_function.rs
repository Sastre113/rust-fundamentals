/*
La primera tarea consiste en corregir problemas de sintaxis en la definición de enumeración para que el código se compile.
*/

// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic
}


// Build a "Car" by using values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    let car: Car = todo!("Create an instance of a `Car` struct");
}

fn main() { 

}