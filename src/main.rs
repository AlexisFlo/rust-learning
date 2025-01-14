fn main() {

    let mut mi_vec = vec![1,2,3,4,5];
    let mut lista_de_nombres = vec!["Alexis", "Arturo", "Alan"];

    println!("Mis valores: {mi_vec:?}");
    println!("Lista de nombres: {lista_de_nombres:?}");

    mi_vec.push(7);
    lista_de_nombres.push("Anayeli");

    println!("Mis valores: {mi_vec:?}");
    println!("Lista de nombres: {lista_de_nombres:?}");
}

