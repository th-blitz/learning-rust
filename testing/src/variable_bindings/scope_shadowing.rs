

fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner : {}", short_lived_binding);
    }

    println!("long lived : {}", long_lived_binding);

    let shadowed_binding = 1;

    {
        println!("before being shodowed : {}", shodowed_binding);
        let shadowed_binding = "abc";
        println!("shdowed in inner block : {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding);
    let shadowed_binding = 2;
    println!("shadowed in outer block : {}", shadowed_binding);
}