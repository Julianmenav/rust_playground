fn main() {

    

    let s = String::from("hello");
    let s = change_passing_ownership(s);
    println!("{s}");

    
    
    let mut s1 = String::from("Hola");
    change_mutable_reference(&mut s1);
    println!("{s1}");


}

// Retorna variable y entrega Ownership
fn change_passing_ownership(mut some_string: String) -> String {
    some_string.push_str(", world");

    some_string
}

// Referencia mutable
fn change_mutable_reference(some_string: &mut String) {
    some_string.push_str(", mundo");
    // String::push_str(some_string, ", mundo");
}