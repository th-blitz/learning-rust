

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("an integer : {:?}", copied_integer);
    println!("a bool : {:?}", a_boolean);
    println!("unit : {:?}", unit);

    let _unused_variable = 3u32;
}