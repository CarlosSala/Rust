// fn is a reserved word, main is the principal function
fn main(){
    /*
    println! is a macro
    because it has a !
    */
    // println!("Hello world");

    // variables let are for default inmutable
    // there is to use mut for that variables are mutables
    // print value x
    let mut x = 10;
    println!("{}", x);
    x = 11;
    println!("{}", x);

    // for const there is to declare type of data
    const MY_CONST: u32 = 100000; // 100_000 --> 100,000
    println!("{}", MY_CONST);

    // change type of data with variable shadowing,
    // it's no possible with mut variable
    let y = "hey";
    let y = y.len();
    println!("{}", y);

    /*
    type data
    -unsigned:
    u8, u16, u32, u64, u128, usize
    -signed (integer):
    i8; i16, i32, i64, i128, isize

    */

    // let z = 10.0






}