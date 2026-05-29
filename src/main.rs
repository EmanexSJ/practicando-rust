use std::io;
// Para usar la constante PI en Ruest se utiliza std::f64::consts::PI;
fn main() {
    // Ejercicio 1)
    // Escribir un algoritmo que permite asignar los numeros 10, 15 y 20 a tres
    // variables numericas. Calcule y muestre el promedio de ellas.
    // fn ejercicio_uno() {
    //     let diez: i32 = 10;
    //     let quince: i32 = 15;
    //     let veinte: i32 = 20;
    //     let promedio: i32 = (diez + quince + veinte) / 3;
    //     println!("El Valor Promedio de las variables diez, quince y veinte es: {promedio}");
    // }
    // ejercicio_uno();
    //
    // Ejercicio 2)
    // Escribir un algoritmo que permita leer un valor real que represente el radio de una
    // circunferencia, Calcule y muestre el perimetro y la superficie de la misma.
    // Recordar que P = 2 * pi * radio y que S = pi * radio * radio
    fn ejercicio_dos() {
        const PI: f64 = std::f64::consts::PI; // Rust provee el numero pi de manera nativa en su libreria estandar
        let mut radio = { String::new() };
        println!(
            "Ingrese el radio de la circunferencia de la que desea saber perimetro y superficie\n"
        );
        io::stdin()
            .read_line(&mut radio)
            .expect("Fallo al leer respuesta.");
        let radio: f64 = radio.trim().parse().expect("Por favor ingrese un número!");
        // Debido a que Rust no permite la conversion implicita de tipos, convertimos explicitamente (cast) el entero (el 2)
        // a un tipo flotante con "as f64" para que coincida con el tipo f64 del numero de coma flotante
        let perimetro = (2 as f64) * PI * radio; // Convertimos explicitamente el entero 2 a f64 usando (2 as f64)
        let superficie = PI * radio * radio;
        println!();
        println!(
            "El Perimetro de una circunferencia de radio: {}, es: {:.2} \n", // Se utiliza ":.N" para mostrar N decimales, dos aqui.
            radio, perimetro
        );
        println!(
            "La Superficie de una circunferencia de radio: {}, es: {:.2} \n",
            radio, superficie
        );
        println!(
            "Gracias por utilizar este software, ojala que su dia se halla compilado correctamente!"
        );
    }
    ejercicio_dos();
}
