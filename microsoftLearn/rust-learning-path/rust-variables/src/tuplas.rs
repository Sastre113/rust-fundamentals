/*
    Rust admite tres tipos de estructura: clásicas, de tupla y de unidad. 
    Estos tipos de estructura admiten diferentes maneras de agrupar y trabajar con los datos.
    
    · Las estructuras de C clásicas son las más utilizadas. 
        Cada campo de la estructura tiene un nombre y un tipo de datos. 
        Una vez definida una estructura clásica, se puede acceder a los campos de 
        la estructura usando la sintaxis <struct>.<field>.
    · Las estructuras de tupla son parecidas a las clásicas, 
        pero sus campos no tienen nombres. A fin de acceder a los campos de 
        una estructura de tupla, usamos la misma sintaxis que 
        para indexar una tupla: <tuple>.<index>. 
        Al igual que con las tuplas, los valores de índice de la 
        estructura de tupla empiezan por cero.
    · Las estructuras de unidad suelen usarse como marcadores. 
        Obtendremos más información sobre por qué las estructuras pueden resultar 
        útiles cuando descubramos la característica rasgos de Rust.
    */
// Classic struct with named fields
struct Student { name: String, level: u8, remote: bool }

// Tuple struct with data types only
struct Grades(char, char, char, char, f32);

// Unit struct
struct Unit;


fn main() {
    // Tuple of length 3
    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);
    
    // Instantiate classic struct, specify fields in random order, or in specified order
    let user_1 = Student { name: String::from("Constance Sharma"), remote: true, level: 2 };
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };

    // Instantiate tuple structs, pass values in same order as types defined
    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_1.name, user_1.level, user_1.remote, mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("{}, level {}. Remote: {}. Grades: {}, {}, {}, {}. Average: {}", 
            user_2.name, user_2.level, user_2.remote, mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}