fn main() {
    // Memoria RAM
    // Stack | Heap
    // Stack -> primitives (i32, str, bool, punteros) | ventajas: es más rapido para lectura
    // Heap -> estructuras más complejas (Vec, HashMap, String) | desventaja: es más rápido para datos complejos

    // Se crea en el Heap el vector pero además se guarda la dirección de memoria del heap en el stack

    let mi_vec: Vec<i32> = Vec::new();
}

