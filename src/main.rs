fn main() {
    // True
    // False

    // clean code
    // Es -> Is/Are
    // Tiene -> Has/Have
    let is_raining = true;
    let has_videogames = true;
    
    if is_raining && has_videogames { // Solo entra si la condición es verdadera
        println!("La persona se queda jugando videojuegos");
    } else { // En el else entra solo si la condición anterior no se cumple
        println!("La persona salio a la calle");
    }
}

