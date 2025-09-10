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

// terdapat beberapa tipe data untuk integer
// 8, 16, 32, 64, 128
// ukuran default bilangan bulat i32 dan desimal f64
#[test]
fn explicit() {
    let age: i32= 15;
    println!("Saya berumur {}", age);
}

#[test]
fn number() {
    let a: i32 = 10;
    println!("Angka {}", a);

    let b: f32 = 10.6;
    println!("Angka {}", b); 
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("Angka {}", a);

    let b: i16 = a as i16;
    println!("Angka {}", b); 

    let b: i32 = b as i32;
    println!("Angka {}", b); 


    let overflow: i64 = 1000000000;
    let temp_overflow: i8 = overflow as i8;

    println!("{}", temp_overflow) 
}

#[test]
fn number_operation() {
    let angka1 = 10;
    let angka2 = 12;
    let hasil = angka1*angka2;

    print!("{}", hasil)
}

#[test]
fn augmented_assingment() {
    let mut angka1 = 10;
    let mut angka2 = 12;
    angka1+=30;
    // angka1-=30;
    // angka1*=30;

    print!("{}", angka1)
}