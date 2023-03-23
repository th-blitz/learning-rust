

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

pub fn run() {

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}< world!", s1);


    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);



    let slice = &s[..2];

    let slice = &s[2..];

    

}