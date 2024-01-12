// Importamos el módulo 'io' del paquete estándar de Rust para manejar la entrada/salida
use std::io;
// Testiamos git and github en Arch que todo este correcto, nuevo commit
// Importamos el módulo 'client' que contiene la definición del cliente
mod client;
use rusqlite::{Connection, Result};

// Función principal
fn main() -> Result<()> {
    // Creamos una variable del tipo string que ocupa todo el espacio que un string puede ocupar
    let usersrt: &str;
    // Inicializamos la variable con un valor
    usersrt = "Malkov";
    // Imprimimos el valor de la variable
    println!("{usersrt}");
    
    // Nuestra variable crística se creará siempre de esta forma. O es que es todo crítico. xD
    // Creamos una cadena de texto dinámica (String) con un contenido específico
    let usrt: String = String::from("String con longitud total del string pasado");
    // Imprimimos la cadena de texto
    println!("{usrt}");

    // Mostramos un menú de opciones
    println!("Selecciona una opción:");
    println!("1. Add Client 1");
    println!("2. Opción 2");
    println!("3. Opción 3");

    // Creamos una variable para almacenar la entrada del usuario
    let mut input = String::new();
    // Leemos la línea de entrada del usuario y manejamos posibles errores
    io::stdin().read_line(&mut input).expect("Error al leer la línea");

    // Parseamos la entrada a un número entero
    let choice: u32 = input.trim().parse().expect("Por favor, ingresa un número");

    // Evaluamos la opción seleccionada
    match choice {
        1 => {
            println!("Elegiste la Opción 1");
            let conn = Connection::open("clientes.db")?;

            client::guardar_en_base_de_datos(&conn)?;
            Ok(())
        }

        2 => {
            println!("Elegiste la Opción 1");
            println!("Opción Dos");
            Ok(())
        }

        _ => {
            println!("Opción no válida");
            Ok(()) // Retornamos Result para indicar que todo está bien
        }
    }
}

