fn main() {
    let a_number; // Declaración de variable. Por defecto en Rust son Inmutables.
    a_number = 5; // Asignación del valor. Esta variable es Inmutable, no se podrá alterar
    println!("Valor de a_number = {}", a_number);

    //a_number += 10; // El compilador daría error

    let mut b_number = 10 + a_number; // Variable declarada como mutable, gracias a 'mut'
    println!("Valor de b_number = {}", b_number);

    /*
        Las variables con el mismo nombre en Rust puede ser remplazadas.
        En otros lenguajes, no se puede declarar en el mismo ambito varias variables
        con el mismo nombre, en Rust si, provocando un reemplazado.
     */
    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num" 
    let shadow_num = shadow_num + 5; 

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2; 

    println!("The number is {}.", shadow_num);
    //let shadow_num; // Esto daría en error, rustc --explain E0282`

    let number: u32 = 14; // Variable de tipo number con un entero de 32 bits.
    println!("The number is {}.", number);

    let number_64 = 4.0;      // compiler infers the value to use the default type f64
    let number_32: f32 = 5.0; // type f32 specified via annotation

    // Addition, Subtraction, and Multiplication
    println!("1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}", 1u32 + 2, 8i32 - 5, 15 * 3);

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    /*
        Booleanos
     */
    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4 ? {}", is_bigger);  


    /*
        String, char y &str
     */

    /*
    Rust admite valores de texto con dos tipos de cadena básicos y un tipo de carácter. Un carácter es un elemento único, mientras que una cadena es una serie de caracteres. 
    Todos los tipos de texto son representaciones UTF-8 válidas.
     */
    let uppercase_s = 'S';
    let lowercase_f = 'f';
    let smiley_face = '😃';
    println!("Esto es una carita --> {}", smiley_face);


    // Specify the data type "char"
    let character_1: char = 'S';
    let character_2: char = 'f';
    
    // Compiler interprets a single item in quotations as the "char" data type
    let smiley_face = '😃';

    // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
    let string_1 = "miley ";

    // Specify the data type "str" with the reference syntax "&str"
    let string_2: &str = "ace";

    println!("{} is a {}{}{}{}.", smiley_face, character_1, string_1, character_2, string_2);
}   

