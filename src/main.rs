fn main() {
    // Switch
    

    let mi_tupla = ("hola", 10);
    match mi_tupla {
        (_,1..=4) => println!("Entra en el caso nro 1 al 4 sin importar el texto"),
        ("hola", 5..=12) => println!("Esta entre el 5 y el 12 y dice hola"), 
        ("hola", _) => println!("Nos interesa verificar si dice hola, no nos importa el numero"), 
        _ => todo!()
    }
    
}

