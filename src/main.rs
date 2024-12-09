use std::{fs::{File, OpenOptions}, io};

//import file ./src/listas.rs
mod listas;

fn main() {
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

                println!(
                    "Encabezado: Número de registros = {}, Primer registro = {}, Último registro = {}",
                    encabezado.numero_de_registros,
                    encabezado.primer_registro,
                    encabezado.ultimo_registro
                );

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
