fn main() {
    // &str, Stack, &[char;n] | &[u8;n]
    // String, Heap, Vec<u8>, Dereferencia a &str

    let mut mi_nombre = "Alexis".to_string();

    mi_nombre.push_str(" Flores");

    let texto_formateado = format!(" eres el usuario número {}", 73);

    mi_nombre.push_str(&texto_formateado);

    println!("{mi_nombre}");
}

