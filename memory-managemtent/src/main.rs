fn test(input: String) {}
fn test_number(input: i32) {}

fn main() {
    // Los valores de las variables en Rust solo pueden pertenecer a una variable
    let mascot = String::from("Luna");

    // Una vez que se asigna una variable a otra
    // la variable original queda invalida,
    // ya que su valor ahora pertenece a la variable nueva
    let cat = mascot;

    println!("{}", cat); // Esto imprime "Luna"
    // print!("{}", mascot); // Esto provocara un error de compilacion

    // La propiedad de las variables de Rust tambien aplica para las funciones
    let message = String::from("value");

    // Una vez que se pasa una variable a una funcion
    // El valor de esta variable se enlaza a la funcion
    // Dejando a la variable invalida
    test(message); // Esto funcionara
    // test(message); // Esto lanzara un error de compilacion

    // En el caso de las variables de tipo simple
    // Sus valores directamente se copian en lugar de volver a enlazar
    // Esto debido a que estos implementan el rasgo Copy
    let number = 2;

    // En casos de variables de tipos simples
    // Estos 2 casos funcionaran bien
    test_number(number);
    test_number(number);

    // Copiar tipos complejos

    let str = String::from("HIIII!!!!");

    // El metodo clone nos permite crear una copia de nuestra variable
    // Por lo que el valor de este no se volvera a enlazar si es que se usa el metodo clone
    test(str.clone()); // Crea una copia en lugar de mover el valor
    test(str); // Aqui se puede utilizar nuevamente debido a que anteriormente se creo una copia y no se movio el valor
    // test(str); // Aqui lanzara un error de compilacion debido a que si se movio el valor en la linea anterior

    loans();
}

fn print_greeting(message: &String) {
    println!("Greeting {}", message);
}

fn change_value(message: &String) {
    // Los tipos referencia no pueden ser modificados
    // A no ser que se especifique que el parametro es
    // &mut String (La variable que se entregara tambien debe setr mutable)
    // message.push_str("!");
}

fn change_value2(message: &mut String) {
    // Los tipos referencia no pueden ser modificados
    // A no ser que se especifique que el parametro es
    // &mut String (La variable que se entregara tambien debe setr mutable)
    message.push_str("!");
}

fn loans() {
    let greeting = String::from("HELLO WORLD");
    let reference = &greeting;

    println!("This is the value of de greeting reference {}", reference);

    let another_greeting = String::from("Hello");

    // En este caso ninguno de estos 2 dara error de compilacion
    // Esto debido a que son referencias al valor y no el valor
    print_greeting(&another_greeting);
    print_greeting(&another_greeting);

    // Esta variable sera mutable desde la funcion
    let mut mutable_greeting = String::from("HIIII");

    change_value2(&mut mutable_greeting);
    change_value2(&mut mutable_greeting);
}
