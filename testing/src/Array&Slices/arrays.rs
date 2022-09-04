use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element : {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}


fn main() {

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let array_2: [i32; 500] = [0;500];

    println!("len of array: {}", arra.len());

    println!("stack allocated array memory size : {}", mem::size_of_val(&array));

    analyze_slice(&array[1..4]);

    let empty_array: [u32; 0] = [];

}