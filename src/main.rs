fn main() {
    // [] -> Primitivos

    let mut lista_vacia: Vec<i32> = Vec::with_capacity(5);

    lista_vacia.push(1);
    lista_vacia.push(2);
    lista_vacia.push(3);
    lista_vacia.push(4);
    lista_vacia.push(5);

    println!("Lista vacía: {lista_vacia:?}, {}, {}", lista_vacia.len(), lista_vacia.capacity());
    lista_vacia.push(6);

    println!("Lista vacía: {lista_vacia:?}, {}, {}", lista_vacia.len(), lista_vacia.capacity());
}

