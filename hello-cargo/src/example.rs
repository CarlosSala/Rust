use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::io;

fn main() {
    println!("¡Bienvenido al Generador de Contraseñas Seguras en Rust!");

    // Solicitar la longitud de la contraseña
    println!("Ingrese la longitud deseada para la contraseña:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let length: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, ingrese un número válido.");
            return;
        }
    };

    // Preguntar si incluir caracteres especiales
    println!("¿Incluir caracteres especiales? (s/n):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Error al leer la entrada");
    let include_specials = input.trim().eq_ignore_ascii_case("s");

    // Generar la contraseña
    let password = generate_password(length, include_specials);
    println!("\nTu contraseña generada es: {}", password);
}

/// Genera una contraseña aleatoria.
/// `length`: Longitud de la contraseña.
/// `include_specials`: Si incluir caracteres especiales.
fn generate_password(length: usize, include_specials: bool) -> String {
    let mut rng = thread_rng();
    let alphanumeric: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    if include_specials {
        let specials = "!@#$%^&*()-_=+[]{}|;:',.<>?/`~";
        let special_count = length / 4; // Usar un cuarto de la longitud para caracteres especiales
        let mut password: Vec<char> = alphanumeric.chars().collect();

        for _ in 0..special_count {
            let index = rng.gen_range(0..password.len());
            let special_char = specials.chars().nth(rng.gen_range(0..specials.len())).unwrap();
            password[index] = special_char;
        }

        password.into_iter().collect()
    } else {
        alphanumeric
    }
}
