fn main() {
    // Switch
    
    let tengo_hambre = true;
    let hay_comida = false;

    match (tengo_hambre, hay_comida) {
        (true, true) => println!("Voy a comer algo"),
        (false, true) => println!("Hay comida pero no tengo hambre"), 
        (true, false) => println!("Tengo hambre pero no hay comida!!"), 
        _ => println!("Pido delivery")
    }
    
}

