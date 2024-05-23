
# Proyecto de Rust: Guía de Inicio

Bienvenido a este proyecto de Rust. Esta guía te ayudará a empezar con Rust, un lenguaje de programación moderno y seguro. Incluiremos pasos para la instalación, creación de un proyecto básico, y recursos adicionales para profundizar en el ecosistema de Rust.

## Instalación de Rust

Para instalar Rust, necesitarás los C++ Build Tools 2013 o versiones más recientes.

1. Instala Rust utilizando `rustup`:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Verifica la instalación:
   ```sh
   rustc --version
   ```

## Crear un Proyecto Nuevo

1. Crea un nuevo proyecto llamado `hello-rust`:
   ```sh
   cargo new hello-rust
   cd hello-rust
   ```
2. Compila el proyecto:
   ```sh
   cargo build
   ```
3. Ejecuta el proyecto:
   ```sh
   cargo run
   ```

## Jueguito

Explora el siguiente juego creado con Rust:
[Juega Crab Tag](https://aaronerhardt.gitlab.io/crab-tag/#slow)

## Recursos y Lecturas Adicionales

### Seguridad y Uso de Unsafe

- [Proyecto RustBelt](https://plv.mpi-sws.org/rustbelt/)
- [Uso de Unsafe en Rust](https://blog.logrocket.com/unsafe-rust-how-and-when-not-to-use-it/)
- [Informe Técnico de la Casa Blanca](https://www.whitehouse.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf)

### Rendimiento

- [Comparativa de Rendimiento entre Lenguajes Compilados: Java, Golang y Rust](https://www.enmilocalfunciona.io/comparativa-de-rendimiento-entre-lenguajes-de-programacion-compilados-java-golang-y-rust/)
- [The Computer Language Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/index.html)
- [Comparaciones de Rendimiento de Lenguajes](https://medium.com/@slimm609/programming-language-performance-comparisons-9a55abad28f4)
- [Eficiencia Energética de Lenguajes de Programación](https://thenewstack.io/which-programming-languages-use-the-least-electricity/)

### Rust vs C++ en Operaciones Complejas

- [Rust and C++: Entre Agujeros Negros y Fractales](https://medium.com/@pepitoscrespo/rust-and-c-between-blackholes-and-fractals-2c1f148cec78)

### Sistemas Operativos en Rust

- [Redox OS](https://www.redox-os.org/)
- [Rust para Linux](https://rust-for-linux.com/)
- [Rust en el Núcleo de Linux](https://www.tomshardware.com/news/rust-in-linux-kernel#:~:text=Adding%20support%20for%20Rust%20adds%20about%2012%2C500%20lines,which%20Rust%20has%20a%20certain%20amount%20of%20interoperability.)
- [Estado del Código Rust en el Núcleo de Linux](https://unix.stackexchange.com/questions/768873/how-much-of-the-linux-kernel-is-currently-coded-in-rust#:~:text=so%200.03%25%20of%20the%20code,phy%20driver%20written%20in%20Rust.)
- [Rust y WebAssembly](https://www.rust-lang.org/es/what/wasm)

## Herramientas y Ecosistema

- [Crates.io: Registro de Paquetes de Rust](https://crates.io/)
- [Comparativa entre Rust y C](https://www.geeksforgeeks.org/rust-vs-c/)
- [Comparativa de Velocidad: Go vs Rust](https://dev.to/mukeshkuiry/go-vs-rust-speed-test-which-one-to-choose-in-2024-1ck)
- [Rust vs Go: Análisis](https://bitfieldconsulting.com/posts/rust-vs-go#:~:text=Rust%20and%20Go%20are%20both%20awesome&text=Rust%20is%20a%20low%2Dlevel,focused%20on%20safety%20and%20performance.&text=Whereas%3A,%2C%20reliable%2C%20and%20efficient%20software.)
- [Ejemplos en Rust](https://doc.rust-lang.org/rust-by-example/)
- [Repositorio de Rust en GitHub](https://github.com/rust-lang/rust?tab=readme-ov-file)
- [Awesome Rust: Proyectos Destacados](https://github.com/rust-unofficial/awesome-rust)
- [Awesome Rust](https://awesome-rust.com/)

## Proyectos Destacados

- [Boytacean: Un Proyecto en Rust](https://boytacean.joao.me/)