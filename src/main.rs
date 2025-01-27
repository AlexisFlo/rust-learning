
fn main() {
    let a: i32 = -42;
    let b: i32 = 56;
    
    println!("Valor absoluto de a: {}", a.abs());
    println!("a como cadena: {}", a.to_string());
    println!("número de bits en 1 en b: {}", b.count_ones());
    println!("Número de bits en 0 en b: {}", b.count_zeros());

    let (sum, overflowed) = a.overflowing_add(b);
    println!("Suma con posible desbordamiento: {} (desbordado: {})", sum, overflowed);

    let wrapped_sum = a.wrapping_add(b);
    println!("Suma con desbordamiento envuelto: {}", wrapped_sum);
}

 
