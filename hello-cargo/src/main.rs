// fn main is the principal function
/*
Esto es un comentario
*/

fn main() {
    println!("Hola, Rust!");

    // variables

    // let are for default inmutable
    let num1 = 10;

    /* println! is a macro because it has a ! */
    println!("Inmutable variable: {}", num1);

    // there is to use let with mut for that variables are mutables
    let mut num2 = 100;
    println!("Mutable variable: {}", num2);

    num2 = 111;
    println!("Mutable variable: {}", num2);

    let mut my_string: &str = "Esto es una cadena de texto";
    println!("{my_string}");

    my_string = "Aquí cambio el valor de la cadena de texto";
    println!("{my_string}");

    // my_string = 6; Error

    let my_string2: String = String::from("Esta es otra cadena de texto");
    println!("{my_string2}");

    /*  // for const there is to declare type of data
    const MY_CONST: u32 = 100000; // 100_000 --> 100,000
    println!("{}", MY_CONST);

    // change type of data with variable shadowing, it's no possible with mut variable
    let y = "hey";
    let y = y.len();
    println!("{}", y); */
}

/*
type data
-unsigned:
u8, u16, u32, u64, u128, usize
-signed (integer):
i8; i16, i32, i64, i128, isize

*/

// let z = 10.0

/*     use std::collections::{HashMap, HashSet};


        let mut my_int = 7;
        my_int = my_int + 4;
        println!("{my_int}");
        println!("{}", my_int - 1);
        println!("{my_int}");

        println!("Este es el valor de la variable my_int: {}", my_int);

        let my_int64: i64 = 7;
        println!("{my_int64}");

        let my_float = 6.5;
        println!("{my_float}");
        // my_float = my_float + my_int; // Error

        let my_float2: f32 = 6.5;
        println!("{my_float2}");

        let mut my_bool = false;
        println!("{my_bool}");
        my_bool = true;
        println!("{my_bool}");

        // Constantes
        const MY_CONST: &str = "Mi propiedad constante";
        println!("{MY_CONST}");

        // Control de flujo
        if my_int == 10 && my_string == "Hola" {
            println!("El valor es 10")
        } else if my_int == 11 || my_string == "Hola" {
            println!("El valor es 11")
        } else {
            println!("El valor no es 10 ni 11")
        }

        // Lista
        let mut my_list = vec!["Brais", "Moure", "@mouredev"];
        my_list.push("Brais");
        println!("{:?}", my_list);
        println!("{}", my_list[1]);

        // Sets
        let mut my_set: HashSet<&str> = vec!["Brais", "Moure", "@mouredev"].into_iter().collect();
        my_set.insert("Brais");
        println!("{:?}", my_set);

        // Mapas
        let mut my_map: HashMap<&str, i32> =
            vec![("Brais", 36), ("Mabequily", 45), ("Demegorash", 96)]
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

        let mut my_counter = 0;
        while my_counter < my_list.len() {
            println!("{}", my_list[my_counter]);
            my_counter += 1;
        }

        // Funciones
        my_function();

        // Estructuras
        let my_struct = MyStruct::new("Brais", 36);
        println!("{} {}", my_struct.name, my_struct.age)
    }

    fn my_function() {
        println!("Esto es una función");
    }

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
} */
