use std::thread;
use std::ptr;

fn main() {
    println!("Hello, world!");
}



// Rust garantiza que y es una referencia válida a x, evitando accesos a memoria no válida.
fn secure_memory() {
    let x = 5;
    let y = &x; // y es una referencia a x
    println!("El valor de y es: {}", y);
}


// Aunque Rust enfatiza la seguridad de la memoria, 
// también permite usar bloques unsafe para 
// realizar operaciones que normalmente estarían prohibidas por el compilador. 
// Esto debe hacerse con cuidado, 
// ya que es responsabilidad del programador garantizar la seguridad en estos casos.
fn boom_memory() {
    let mut x: i32 = 10;
    let y: *mut i32 = &mut x; // y es un puntero a x

    unsafe {
        *y = 20; // Modificando x a través de y en un bloque unsafe
    }

    println!("El valor de x es: {}", x);
}


// Rust es capaz de manejar colecciones grandes 
// y realizar operaciones sobre ellas rápidamente.
fn vectorcitos() {
    let mut vec: Vec<i32> = Vec::with_capacity(1000);
    for i in 0..1000 {
        vec.push(i);
    }
    let sum: i32 = vec.iter().sum();
    println!("La suma de los elementos del vector es: {}", sum);
}



// Rust facilita la programación concurrente, 
// lo que permite ejecutar tareas en paralelo para mejorar el rendimiento.


fn hilitos() {
    let handles: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || {
            println!("Hilo número: {}", i);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}


// Utilizamos un bucle para crear 10 hilos. Cada hilo imprime su número (del 0 al 9).
// La función thread::spawn crea un nuevo hilo, ejecutando el código que se pasa dentro del move closure. 
// El move garantiza que el hilo tome posesión de cualquier variable utilizada dentro del closure.

fn concurrencia_segura() {
    let handles: Vec<_> = (0..10).map(|i| {
        thread::spawn(move || {
            println!("Hilo número: {}", i);
        })
    }).collect();

// Los manejadores de los hilos (handles) se recogen en un vector.
// Después de crear los hilos, el programa espera a que todos terminen su ejecución utilizando handle.join().unwrap(). 
// Esto asegura que el programa principal no termine antes de que los hilos completen su trabajo.
    for handle in handles {
        handle.join().unwrap();
    }
}




fn puntero_loco() {
    let mut x = 5;
    // convierte una referencia mutable a x en un puntero crudo mutable (*mut i32).
    let raw = &mut x as *mut i32;
    // El bloque unsafe permite realizar operaciones que el compilador normalmente no permitiría por motivos de seguridad.
    // *raw = 10; modifica el valor apuntado por raw directamente, cambiando x a 10.
    unsafe {
        *raw = 10;
    }
    println!("El valor de x es: {}", x);
}


// fn divide(dividendo: f64, divisor: f64) -> Option<f64> define una función que retorna un Option<f64>.
// Si el divisor es 0.0, retorna None, indicando una división inválida.
// De lo contrario, retorna Some(dividendo / divisor), proporcionando el resultado de la división.
fn divide(dividendo: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividendo / divisor)
    }
}

// La función main llama a divide(10.0, 0.0) y usa una expresión match para manejar el resultado.
// Si la división es válida (Some(result)), imprime el resultado.
// Si es inválida (None), imprime un mensaje de error indicando división por cero.
fn robusto_y_fuerte() {
    match divide(10.0, 0.0) {
        Some(result) => println!("Resultado: {}", result),
        None => println!("Error: División por cero"),
    }
}




fn sum(a: i32, b: i32) -> i32 {
    a + b
}

