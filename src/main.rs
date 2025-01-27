
fn main() {
    let saludo: &str = "Hola, mundo!";

    println!("Longitud: {}", saludo.len());
    println!("¿Esta vacío? {}", saludo.is_empty());
    println!("¿Contine 'mundo'? {}", saludo.contains("mundo"));

    for parte in saludo.split(", ") {
        println!("Parte: {}", parte);
    }

    println!("En mayúsculas: {}", saludo.to_uppercase());
}

 
