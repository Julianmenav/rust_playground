// Definición del enum Color, que tiene variantes con datos asociados
enum Color {
    Red(u8, u8, u8), // RGB
    Green(u8, u8, u8),
    Blue(u8, u8, u8),
    Custom(String),
}

fn main() {
    // Instancias del enum Color
    let color1 = Color::Red(255, 0, 0);
    let color2 = Color::Custom(String::from("Fuchsia"));

    // COMO SACAR EL VALOR DE NUESTRO ENUM.
    // match
    match color1 {
        Color::Red(r, g, b) => println!("Color Rojo: RGB({}, {}, {})", r, g, b),
        Color::Custom(name) => println!("Color Personalizado: {}", name),
        _ => {} // other te permite sacar el valor de un enum.
    }

    // let if
    if let Color::Custom(name) = color2 {
        println!("Encontrado un color personalizado: {}", name);
    } else {
        println!("No es un color personalizado.");
    }

    // EJEMPLO OTHER
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Ha salido un 3") ,
        7 => println!("Ha salido un 7") ,
        other => println!("Ha salido otro numero {}", other) ,
    }    


    // LO MISMO PERO CON EL ENUM OPTION
    let some_value: Option<i32> = Some(10);
    let no_value: Option<i32> = None;

    match some_value {
        Some(val) => println!("Valor presente: {}", val),
        None => println!("No hay valor"),
    }

    if let Some(val) = no_value {
        println!("Valor presente: {}", val);
    } else {
        println!("No hay valor en la variable no_value");
    }
}
