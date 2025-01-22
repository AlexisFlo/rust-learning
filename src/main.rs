fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("El valor de x en el contexto interno es: {x}"); 
    }
    println!("El valor de x en el contexto externo es: {x}");
}


