fn main() {
    let mi_lista_de_usuarios = vec![
        ("Alexis", "Flores"),
        ("Karla", "Garcia"),
        ("Arturo", "Pascual"),
        ("Alexa", "Gomez"),
    ];

    // foreach
    for (indice, (nombre, apellido)) in mi_lista_de_usuarios.iter().enumerate() {
        println!("El numero del usuario es {indice}");
        println!("El nombre y apellido del usuario es {nombre} {apellido}");
    }
}

