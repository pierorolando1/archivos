use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};

#[derive(Debug, Clone)]
pub struct Registro {
    clave: String,
    valor: String,
}

impl Registro {
    // Constructor para crear un nuevo registro
    fn new(clave: String, valor: String) -> Self {
        Self { clave, valor }
    }

    // Convierte un registro a formato de texto
    fn to_string(&self) -> String {
        format!("{}|{}", self.clave, self.valor)
    }

    // Crea un registro a partir de una línea de texto
    fn from_string(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            Some(Self {
                clave: parts[0].to_string(),
                valor: parts[1].to_string(),
            })
        } else {
            None
        }
    }
}

pub struct TablaHash {
    archivo: String,
    tamanio: usize,
}

impl TablaHash {
    // Constructor para inicializar la tabla hash
    pub fn new(tamanio: usize, archivo: &str) -> Self {
        let mut tabla_hash = Self {
            archivo: archivo.to_string(),
            tamanio,
        };
        tabla_hash.inicializar_archivo();
        tabla_hash
    }

    // Crea un archivo vacío con espacio reservado para las listas encadenadas
    fn inicializar_archivo(&self) {
        let archivo = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.archivo)
            .expect("No se pudo abrir el archivo");

        // Inicializa cada índice con una línea vacía
        for _ in 0..self.tamanio {
            writeln!(&archivo, "").expect("Error al inicializar el archivo");
        }
    }

    // Función hash simple basada en la suma de bytes
    fn hash(&self, clave: &str) -> usize {
        let hash: usize = clave.bytes().map(|b| b as usize).sum();
        hash % self.tamanio
    }

    // Inserta un nuevo registro en la tabla hash
    pub fn insertar(&self, clave: String, valor: String) {
        let indice = self.hash(&clave);
        let registro = Registro::new(clave.clone(), valor);

        let mut archivo = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.archivo)
            .expect("No se pudo abrir el archivo");

        let mut buffer = String::new();
        archivo
            .seek(SeekFrom::Start((indice * 50) as u64))
            .expect("No se pudo mover el cursor");
        archivo.read_to_string(&mut buffer).expect("No se pudo leer el archivo");

        // Convierte las líneas leídas en registros
        let mut registros: Vec<Registro> = buffer
            .lines()
            .filter_map(|line| Registro::from_string(line))
            .collect();

        // Verifica si la clave ya existe
        if registros.iter().any(|r| r.clave == clave.clone()) {
            println!("La clave ya existe, no se insertará.");
        } else {
            registros.push(registro);

            // Reescribe la lista actualizada en el archivo
            archivo.seek(SeekFrom::Start((indice * 50) as u64)).expect("No se pudo mover el cursor");
            let nuevos_datos = registros
                .into_iter()
                .map(|registro| registro.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            archivo.write_all(nuevos_datos.as_bytes()).expect("Error al escribir en el archivo");
        }
    }

    // Busca un registro por su clave
    pub fn buscar(&self, clave: &str) -> Option<Registro> {
        let indice = self.hash(clave);
        let mut archivo = OpenOptions::new()
            .read(true)
            .open(&self.archivo)
            .expect("No se pudo abrir el archivo");

        let mut buffer = String::new();
        archivo
            .seek(SeekFrom::Start((indice * 50) as u64))
            .expect("No se pudo mover el cursor");
        archivo.read_to_string(&mut buffer).expect("No se pudo leer el archivo");

        // Busca la clave entre los registros del índice
        buffer
            .lines()
            .filter_map(|line| Registro::from_string(line))
            .find(|registro| registro.clave == clave)
    }

    // Elimina un registro por su clave
    pub fn eliminar(&self, clave: &str) -> bool {
        let indice = self.hash(clave);
        let mut archivo = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.archivo)
            .expect("No se pudo abrir el archivo");

        let mut buffer = String::new();
        archivo
            .seek(SeekFrom::Start((indice * 50) as u64))
            .expect("No se pudo mover el cursor");
        archivo.read_to_string(&mut buffer).expect("No se pudo leer el archivo");

        // Filtra la clave a eliminar
        let mut registros: Vec<Registro> = buffer
            .lines()
            .filter_map(|line| Registro::from_string(line))
            .collect();

        let len_antes = registros.len();
        registros.retain(|registro| registro.clave != clave);

        // Si el tamaño cambió, se elimina el registro
        if registros.len() < len_antes {
            archivo.seek(SeekFrom::Start((indice * 50) as u64)).expect("No se pudo mover el cursor");
            let nuevos_datos = registros
                .into_iter()
                .map(|registro| registro.to_string())
                .collect::<Vec<_>>()
                .join("\n");

            archivo.write_all(nuevos_datos.as_bytes()).expect("Error al escribir en el archivo");
            return true;
        }

        false
    }
}
