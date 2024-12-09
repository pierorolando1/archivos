use std::{fs::{File, OpenOptions}, io};

//import file ./src/listas.rs
mod listas;
mod hasheando;




fn main() {
    let tabla_hash = hasheando::TablaHash::new(10, "tabla_hash.txt");

    loop {
        println!("\nMenú:");
        println!("1. Insertar");
        println!("2. Buscar");
        println!("3. Eliminar");
        println!("4. Salir");
        println!("Seleccione una opción:");

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Error al leer la entrada");
        let opcion = opcion.trim();

        match opcion {
            "1" => {
                let mut clave = String::new();
                let mut valor = String::new();

                println!("Ingrese la clave:");
                io::stdin().read_line(&mut clave).expect("Error al leer la clave");
                println!("Ingrese el valor:");
                io::stdin().read_line(&mut valor).expect("Error al leer el valor");

                tabla_hash.insertar(clave.trim().to_string(), valor.trim().to_string());
                println!("Intento de inserción completado.");
            }
            "2" => {
                let mut clave = String::new();
                println!("Ingrese la clave a buscar:");
                io::stdin().read_line(&mut clave).expect("Error al leer la clave");

                match tabla_hash.buscar(clave.trim()) {
                    Some(registro) => println!("Registro encontrado: {:?}", registro),
                    None => println!("Registro no encontrado."),
                }
            }
            "3" => {
                let mut clave = String::new();
                println!("Ingrese la clave a eliminar:");
                io::stdin().read_line(&mut clave).expect("Error al leer la clave");

                if tabla_hash.eliminar(clave.trim()) {
                    println!("Registro eliminado correctamente.");
                } else {
                    println!("No se encontró el registro a eliminar.");
                }
            }
            "4" => {
                println!("Saliendo del programa...");
                break;
            }
            _ => println!("Opción no válida."),
        }
    }
}
