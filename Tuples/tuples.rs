

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    return (boolean, integer);
}


fn main() {

    let long_tuple = (1u8, 2u16, 3u32, 'a', true);

    println!("long tuple first value : {first_value}", first_value = long_tuple.0);

    let pair: (i32, bool) = (23, true);

    println!("pair : {:?}", pair);
    println!("reversed pair: {:?}", reverse(pair));

    println!("one element tuple must be created with a comma : {:?}", (23i32,));

}