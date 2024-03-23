# Primeros pasos

## Uso de crates y bibliotecas de Rust

La biblioteca estándar de Rust, std, contiene código reutilizable para las definiciones y operaciones fundamentales de los programas de Rust. Esta biblioteca tiene definiciones para tipos de datos principales como, por ejemplo, String y Vec<T>, operaciones para primitivas de Rust, código para funciones de macro usadas con frecuencia, compatibilidad con acciones de entrada y salida, y muchas otras áreas de funcionalidad.

Hay decenas de miles de bibliotecas y crates de terceros disponibles para su uso en los programas Rust; para acceder a la mayoría de ellas se puede usar el repositorio de crates de terceros de Rust, crates.io. Más adelante veremos cómo acceder a estos crates desde nuestro proyecto, pero por ahora estas son algunas de las crates que se usan en los ejercicios de programación:

* std: Biblioteca estándar de Rust. En los ejercicios de Rust, verá que aparecen los siguientes módulos:
  * std::collections: definiciones de tipos de colección, como HashMap.
  * std::env: Funciones para trabajar con el entorno.
  * std::fmt: Funcionalidad para controlar el formato de salida.
  * std::fs: Funciones para trabajar con el sistema de archivos.
  * std::io: Definiciones y funcionalidad para trabajar con entradas y salidas.
  * std::path: definiciones y funciones que permiten trabajar con datos de ruta de acceso del sistema de archivos.

* structopt: crate de terceros para analizar argumentos de línea de comandos fácilmente.
* chrono: crate de terceros para controlar los datos de fecha y hora.
* regex: crate de terceros para trabajar con expresiones regulares.
* serde: crate de terceros con operaciones de serialización y deserialización de estructuras de datos de Rust.

De manera predeterminada, la biblioteca std está disponible para todos los crates de Rust. Para acceder al código reutilizable en un crate o biblioteca, implementamos la palabra clave use. Con la palabra clave use, el código del crate o biblioteca se "incluye en el ámbito" para que pueda acceder a las definiciones y funciones en el programa. Se accede a la biblioteca estándar en instrucciones use con la ruta std, como en use std::fmt. Se accede a otros crates o bibliotecas con su nombre, como use regex::Regex.


## Creación y administración de proyectos con Cargo

Aunque se puede usar el compilador de Rust (rustc) directamente para crear crates, en la mayoría de los proyectos se usa la herramienta de compilación de Rust y un administrador de dependencias llamado Cargo.

Cargo hace gran cantidad de cosas, entre las que se incluyen las siguientes:

*  Crear nuevas plantillas de proyecto con el comando cargo new.
*  Compilar un proyecto con el comando cargo build.
*  Compilar y ejecutar un proyecto con el comando cargo run.
*  Probar un proyecto con el comando cargo test.
*  Comprobar los tipos de proyecto con el comando cargo check.
*  Compilar la documentación de un proyecto con el comando cargo doc.
*  Publique una biblioteca para crates.io con el comando cargo publish.
*  Para agregar crates dependientes a un proyecto, agregue el nombre del crate al archivo Cargo.toml.


