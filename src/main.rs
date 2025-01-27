fn main() {
    // Refinamiento de Tipos
    let spaces = "  ";
    let spaces = spaces.len();
    println!("Longitud de la cadena: {spaces}");

    // Transformaciones intermedias
    let price = 100;
    let price = price * 2;
    let price = price - 10;
    println!("The price is: {price}");

    // Evitar mutabilidad
    // let mut count = 1;
    // count += 1;
    // se puede escribir como ->
    let count = 1;
    let count = count + 1;
    println!("The count is in: {count}");
}
// doing a web server 
 
