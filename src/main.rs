fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello test")
}

#[test]
fn test_variable() {
    let mut name= "joko";
    println!("Hello {}", name);

    name = "marni";
    println!("Hello {}", name)
}
#[test]
fn static_typing() {
    let name= "joko";
    println!("Hello {}", name);

    // name = 20;
    println!("Hello {}", name)
}

#[test]
fn shadowing() {
    let name= "joko";
    println!("Hello {}", name);

    let name = "budiono";
    println!("Hello {}", name)
}

