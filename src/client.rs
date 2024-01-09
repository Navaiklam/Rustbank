// client.rs

// Importamos el módulo 'io' del paquete estándar de Rust para manejar la entrada/salida
use std::io;
// Importamos el módulo 'Connection' y 'Result' de la biblioteca 'rusqlite'
use rusqlite::{Connection, Result};
// Importamos el tipo 'Uuid' desde la biblioteca 'uuid'
use uuid::Uuid;

// Definimos una estructura llamada 'Client' con campos públicos (id, name, email, age)
pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: u32,
}

// Implementamos métodos para la estructura 'Client'
impl Client {
    // Nueva función para crear un cliente a partir de la entrada del usuario
    pub fn create_client() -> Client {
        // Solicitamos al usuario que ingrese el nombre del cliente
        println!("Ingrese el nombre del cliente:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Error al leer la línea");

        // Solicitamos al usuario que ingrese la edad del cliente
        println!("Ingrese la edad del cliente:");
        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Error al leer la línea");
        // Convertimos la cadena de edad a un número entero (u32)
        let age: u32 = age_str.trim().parse().expect("Por favor, ingresa un número");

        // Solicitamos al usuario que ingrese el correo electrónico del cliente
        println!("Ingrese el email del cliente:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Error al leer la línea");

        // Creamos el cliente con un ID único generado por Uuid::new_v4()
        Client {
            id: Uuid::new_v4(),
            name: name.trim().to_string(),
            email: email.trim().to_string(),
            age,
        }
    }
}

// Definimos una función pública llamada 'guardar_en_base_de_datos' que toma una conexión a la base de datos como argumento y devuelve un Result
pub fn guardar_en_base_de_datos(conn: &Connection) -> Result<()> {
    // Creamos una tabla llamada 'clients' si no existe
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clients (
            id TEXT PRIMARY KEY,
            name TEXT,
            email TEXT,
            age INTEGER
        )",
        [],
    )?;
    // La interrogación (?) al final indica que estamos manejando errores utilizando el mecanismo Result

    // Creamos un nuevo cliente utilizando la función 'create_client' del módulo 'client'
    let new_client = Client::create_client();

    // Insertamos el nuevo cliente en la tabla 'clients' de la base de datos
    conn.execute(
        "INSERT INTO clients (id, name, email, age) VALUES (?, ?, ?, ?)",
        [
            &new_client.id.to_string(),    // Convertimos el ID del cliente a una cadena de texto
            &new_client.name,              // Nombre del cliente
            &new_client.email,             // Correo electrónico del cliente
            &(new_client.age.to_string()), // Convertimos la edad del cliente a una cadena de texto y luego a una referencia
        ],
    )?;
    // La interrogación (?) al final indica que estamos manejando errores utilizando el mecanismo Result

    // Puedes realizar otras operaciones con el cliente aquí, si es necesario

    // Retornamos Result con Ok(()) para indicar que todo se ejecutó correctamente
    Ok(())
}

