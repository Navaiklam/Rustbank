// client.rs
use std::io;
use uuid::Uuid; // Añade esta línea para importar Uuid

pub struct Client {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub age: u32,

}

impl Client {
    // Nueva función para crear un cliente a partir de la entrada del usuario
    pub fn create_client() -> Client {
        // nombre al usuario
        println!("Ingrese el nombre del cliente:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Error al leer la línea");

        // edad al usuario
        println!("Ingrese la edad del cliente:");
        let mut age_str = String::new();
        io::stdin().read_line(&mut age_str).expect("Error al leer la línea");
        let age: u32 = age_str.trim().parse().expect("Por favor, ingresa un número");

        // Pedimos el email
        println!("Ingrese el email del cliente:");
        let mut email = String::new();
        io::stdin().read_line(&mut email).expect("Error al leer la línea");

        // Crear el cliente con un ID único
        Client {
            id: Uuid::new_v4(),
            name: name.trim().to_string(),
            email: email.trim().to_string(),
            age,
        }
    }
}
