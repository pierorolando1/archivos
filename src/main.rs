use std::{fs::{File, OpenOptions}, io};

//import file ./src/listas.rs
mod listas;
mod hasheando;
mod arboles;


fn main3() -> Result<(), Box<dyn std::error::Error>> {
    // Crear un árbol binario
    let mut tree = arboles::Node::new(10, "Root".to_string());
    tree.insert(5, "Left".to_string());
    tree.insert(15, "Right".to_string());

    // Guardar el árbol en un archivo binario
    let filename = "binary_tree.bin";
    arboles::save_tree_to_binary(&tree, filename)?;
    println!("Árbol guardado en '{}'.", filename);

    // Cargar el árbol desde el archivo binario
    let loaded_tree = arboles::load_tree_from_binary(filename)?;
    println!("Árbol cargado: {:?}", loaded_tree);

    Ok(())
}


fn main2() {
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



fn main1() {

    let filename = "lista_enlazada.txt";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();

    let mut encabezado = if file.metadata().unwrap().len() == 0 {
        let encabezado = listas::Encabezado::new();
        listas::escribir_encabezado(&mut file, &encabezado);
        encabezado
    } else {
        listas::leer_encabezado(&mut file)
    };

    loop {
        println!("1. Agregar registro");
        println!("2. Mostrar registros");
        println!("3. Salir");

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).unwrap();
        let opcion: i32 = opcion.trim().parse().unwrap();

        match opcion {
            1 => {
                println!("Ingrese el nombre del registro:");
                let mut nombre = String::new();
                io::stdin().read_line(&mut nombre).unwrap();
                let nombre = nombre.trim().to_string();

                let nuevo_registro = listas::Registro::new(
                    encabezado.numero_de_registros + 1,
                    nombre,
                    -1,
                    encabezado.ultimo_registro,
                );

                if encabezado.numero_de_registros == 0 {
                    encabezado.primer_registro = nuevo_registro.numero_de_registro;
                } else {
                    let mut registros = listas::leer_registros(&mut file);
                    if let Some(ultimo) = registros.last_mut() {
                        ultimo.siguiente_registro = nuevo_registro.numero_de_registro;
                    }

                    file.set_len(0).unwrap();
                    listas::escribir_encabezado(&mut file, &encabezado);
                    for registro in registros {
                        listas::escribir_registro(&mut file, &registro);
                    }
                }

                listas::escribir_registro(&mut file, &nuevo_registro);
                encabezado.ultimo_registro = nuevo_registro.numero_de_registro;
                encabezado.numero_de_registros += 1;
                listas::escribir_encabezado(&mut file, &encabezado);
            }
            2 => {
                let registros = listas::leer_registros(&mut file);
                for registro in registros {
                    println!(
                        "Registro {}: Nombre = {}, Siguiente = {}, Anterior = {}",
                        registro.numero_de_registro,
                        registro.nombre,
                        registro.siguiente_registro,
                        registro.anterior_registro
                    );
                }
            }
            3 => break,
            _ => println!("Opción no válida"),
        }
    }
}


fn main() {
    //Create a menu to select the program to run
    loop {
        println!("\nMenú:");
        println!("1. Listas");
        println!("2. Tablas hash");
        println!("3. Árboles binarios");
        println!("4. Salir");
        println!("Seleccione una opción:");

        let mut opcion = String::new();
        io::stdin().read_line(&mut opcion).expect("Error al leer la entrada");
        let opcion = opcion.trim();

        match opcion {
            "1" => main1(),
            "2" => main2(),
            "3" => main3().unwrap(),
            "4" => {
                println!("Saliendo del programa...");
                break;
            }
            _ => println!("Opción no válida."),
        }
    }
}