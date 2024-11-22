// fn main is the principal function
/*
Esto es un comentario
*/

use std::collections::{HashMap, HashSet};

fn main() {
    println!("Hola, Rust!");

    // variables

    // let is inmutable for default
    let num1: i32 = 10;

    // println! is a macro because it has a !
    println!("Inmutable variable: {}", num1);

    // with mut the variables are mutables
    let mut num2: i32 = 100;
    println!("Mutable variable: {}", num2);

    num2 += 1;
    println!("Mutable variable: {}", num2);

    let mut my_string: &str = "This is a string";
    println!("{my_string}");

    my_string = "The string change";
    println!("{my_string}");

    // my_string = 6; Error

    let my_string2: String = String::from("This is another change");
    println!("{my_string2}");

    // for const there is to declare type of data
    const MY_CONST: u32 = 100000; // 100_000 --> 100,000
    println!("{}", MY_CONST);

    // change type of data with variable shadowing, it's no possible with mut variable
    let y: &str = "hey";
    let y: usize = y.len();
    println!("{}", y);

    /*
    type data
    -unsigned:
    u8, u16, u32, u64, u128, usize
    -signed (integer):
    i8; i16, i32, i64, i128, isize
    */

    let my_float: f64 = 6.5;
    println!("{my_float}");
    // my_float = my_float + my_int; // Error

    let my_float2: f32 = 6.5;
    println!("{my_float2}");

    let mut my_bool: bool = false;
    println!("{my_bool}");
    my_bool = true;
    println!("{my_bool}");

    // flow control
    let my_int: i32 = 11;
    println!("{my_int}");

    if my_int == 10 && my_string == "Hola" {
        println!("El valor es 10")
    } else if my_int == 11 || my_string == "Hola" {
        println!("El valor es 11")
    } else {
        println!("El valor no es 10 ni 11")
    }

    // Lists
    let mut my_list: Vec<&str> = vec!["Carlitros", "developer", "@dev"];
    my_list.push("Carlitros");
    println!("{:?}", my_list);
    println!("{}", my_list[1]);

    // Sets
    let mut my_set: HashSet<&str> = vec!["Carlitros", "developer", "@dev"].into_iter().collect();
    // no insert repeat element
    my_set.insert("Carlitros");
    my_set.insert("Carlitros2");
    println!("{:?}", my_set);

    // Maps
    let mut my_map: HashMap<&str, i32> =
        vec![("Carlitros", 36), ("Mabequily", 45), ("Demegorash", 96)]
            .into_iter()
            .collect();
    my_map.insert("lemi_n_n", 26);
    println!("{:?}", my_map);

    // Bucles
    for value in &my_list {
        println!("{}", value);
    }

    for value in &my_set {
        println!("{}", value);
    }

    for (key, value) in &my_map {
        println!("{} {}", key, value);
    }

    // Bucle for
    for my_number in 1..5 {
        // Range 1 to 4
        println!("Number: {}", my_number);
    }

    for element in vec!["a", "b", "c"] {
        println!("Element: {}", element);
    }

    print!("with while: \n");
    let mut my_counter: usize = 0;
    while my_counter < my_list.len() {
        println!("{}", my_list[my_counter]);
        my_counter += 1;
    }

    // functions

    my_function();

    let result: i32 = sum(2, 13);
    println!("Result of sum is: {}", result);

    // structs

    // Estructuras
    let my_struct: MyStruct = MyStruct::new("Carlos", 28);

    println!("Since MyStruct: {} {}", my_struct.name, my_struct.age);

    struct MyStruct {
        name: String,
        age: i32,
    }

    impl MyStruct {
        fn new(name: &str, age: i32) -> MyStruct {
            MyStruct {
                name: String::from(name),
                age,
            }
        }
    }
}

fn my_function() {
    println!("Esto es una función");
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// let z = 10.0
