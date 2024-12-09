use std::fs::File;
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct Encabezado {
    pub numero_de_registros: i32,
    pub primer_registro: i32,
    pub ultimo_registro: i32,
}

#[derive(Debug)]
pub struct Registro {
    pub numero_de_registro: i32,
    pub nombre: String,
    pub siguiente_registro: i32,
    pub anterior_registro: i32,
}

impl Encabezado {
    pub fn new() -> Self {
        Encabezado {
            numero_de_registros: 0,
            primer_registro: -1,
            ultimo_registro: -1,
        }
    }
}

impl Registro {
    pub fn new(numero_de_registro: i32, nombre: String, siguiente_registro: i32, anterior_registro: i32) -> Self {
        Registro {
            numero_de_registro,
            nombre,
            siguiente_registro,
            anterior_registro,
        }
    }
}

pub fn escribir_encabezado(file: &mut File, encabezado: &Encabezado) {
    file.seek(SeekFrom::Start(0)).unwrap();
    let data = format!("{} {} {}\n", encabezado.numero_de_registros, encabezado.primer_registro, encabezado.ultimo_registro);
    file.write_all(data.as_bytes()).unwrap();
}

pub fn leer_encabezado(file: &mut File) -> Encabezado {
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut buffer = String::new();
    let mut reader = io::BufReader::new(file);
    reader.read_line(&mut buffer).unwrap();
    let parts: Vec<i32> = buffer.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    Encabezado {
        numero_de_registros: parts[0],
        primer_registro: parts[1],
        ultimo_registro: parts[2],
    }
}

pub fn escribir_registro(file: &mut File, registro: &Registro) {
    file.seek(SeekFrom::End(0)).unwrap();
    let data = format!("{} {} {} {}\n", registro.numero_de_registro, registro.nombre, registro.siguiente_registro, registro.anterior_registro);
    file.write_all(data.as_bytes()).unwrap();
}

pub fn leer_registros(file: &mut File) -> Vec<Registro> {
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let lines: Vec<&str> = buffer.lines().skip(1).collect();
    let mut registros = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        registros.push(Registro {
            numero_de_registro: parts[0].parse().unwrap(),
            nombre: parts[1].to_string(),
            siguiente_registro: parts[2].parse().unwrap(),
            anterior_registro: parts[3].parse().unwrap(),
        });
    }
    registros
}