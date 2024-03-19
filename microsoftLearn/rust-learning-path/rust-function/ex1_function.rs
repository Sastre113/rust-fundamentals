/*
La primera tarea consiste en corregir problemas de sintaxis en la definición de enumeración para que el código se compile.
*/

// Declare Car struct to describe vehicle with four named fields
#[derive(Debug)]
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


// Build a "Car" by usin
// values from the input arguments
// - Color of car (String)
// - Transmission type (enum value)
// - Convertible (boolean, true if car is a convertible)
fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {

    // Use the values of the input arguments
    // All new cars always have zero mileage
    return Car {color, transmission, convertible, mileage: 0};
}

fn main() { 
  let myCar = car_factory(String::from("Red"), Transmission::Manual, true);
  println!("{:?}", myCar);

  let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
  println!("Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

  car = car_factory(String::from("Silver"), Transmission::Automatic, true);
  println!("Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);

  car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
  println!("Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}", car.color, car.transmission, car.convertible, car.mileage);    
}
