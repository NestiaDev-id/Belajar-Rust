mod karyawan;

fn main() {
    println!("Hello, world!");

    let mut joko = karyawan::Karyawan::new(String::from("Joko"), 5000000.0);

    // Mengakses gaji menggunakan getter
    println!("Gaji awal Joko: {}", joko.get_salary());

    // Mencoba mengubah gaji dengan setter
    joko.set_salary(6500000.0);
    println!("Gaji baru Joko: {}", joko.get_salary());

    // Mencoba mengubah gaji dengan nilai tidak valid
    joko.set_salary(-1000000.0); // Output: Gaji tidak bisa negatif!
    println!("Gaji terakhir Joko: {}", joko.get_salary());
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

// If-Else
#[test]
fn if_else_example() {
    let number = 7;
    if number < 5 {
        println!("Kondisi benar, angka lebih kecil dari 5");
    } else {
        println!("Kondisi salah, angka lebih besar atau sama dengan 5");
    }
}

// While Loop
#[test]
fn while_loop_example() {
    let mut count = 0;
    while count < 3 {
        println!("Perulangan while ke-{}", count);
        count += 1;
    }
}

// Loop (mirip do-while, dengan break)
#[test]
fn loop_with_break() {
    let mut counter = 0;
    loop {
        println!("Perulangan loop ke-{}", counter);
        counter += 1;
        if counter == 3 {
            break; // Menghentikan perulangan
        }
    }
}

// For Loop
#[test]
fn for_loop_example() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Nilai elemen: {}", element);
    }

    // For loop dengan range
    for number in 1..4 {
        println!("Angka dalam range: {}", number);
    }
}

#[test]
fn match_example() {
    let number = 10;

    match number {
        1 => println!("Angka adalah satu"),
        2 | 3 => println!("Angka adalah dua atau tiga"),
        4..=10 => println!("Angka ada di antara 4 dan 10"),
        _ => println!("Angka adalah yang lain"),
    }
}

#[test]
fn array_statis() {
    // array 5 elemen integer.
    let angka: [i32; 5] = [10, 20, 30, 40, 50];

    println!("Elemen pertama: {}", angka[0]); // Output: Elemen pertama: 10
    println!("Elemen terakhir: {}", angka[4]); // Output: Elemen terakhir: 50
}

#[test]
fn array_inferensi() {
    let nilai = [1, 2, 3, 4];
    println!("Nilai array: {:?}", nilai); // Output: Nilai array: [1, 2, 3, 4]

    let nol = [0; 5]; // Array dengan 5 elemen, semuanya 0.
    println!("Array nol: {:?}", nol); // Output: Array nol: [0, 0, 0, 0, 0]
}

#[test]
fn array_modifikasi_dan_iterasi() {
    let mut item = [100, 200, 300];

    // Mengubah nilai elemen pada indeks 1.
    item[1] = 250;
    println!("Array setelah modifikasi: {:?}", item); // [100, 250, 300]

    // looping melalui setiap elemen.
    println!("Looping melalui array:");
    for (i, &angka) in item.iter().enumerate() {
        println!("Indeks {} -> Nilai {}", i, angka);
    }
    // Output:
    // Indeks 0 -> Nilai 100
    // Indeks 1 -> Nilai 250
    // Indeks 2 -> Nilai 300
}
