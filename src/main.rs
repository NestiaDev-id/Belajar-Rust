mod karyawan;

fn main() {
    println!("Hello, world!");

    let mut joko = karyawan::Karyawan::new(String::from("Joko"), 5000000.0);
    // let karyawan = Karyawan::new(String::from("John"), 50000.0);

    // Mengakses gaji menggunakan getter
    println!("Gaji awal Joko: {}", joko.get_salary());

    // Mencoba mengubah gaji dengan setter
    joko.set_salary(6500000.0);
    println!("Gaji baru Joko: {}", joko.get_salary());

    // Mencoba mengubah gaji dengan nilai tidak valid
    joko.set_salary(-1000000.0); // Output: Gaji tidak bisa negatif!
    println!("Gaji terakhir Joko: {}", joko.get_salary());

    println!("Nama karyawan: {}", joko.get_name());

}

#[test]
fn hello_test() {
    println!("Hello test")
}

#[test]
fn test_variable() {
    let mut name = "joko";
    println!("Hello {}", name);

    name = "marni";
    println!("Hello {}", name)
}
#[test]
fn static_typing() {
    let name = "joko";
    println!("Hello {}", name);

    // name = 20;
    println!("Hello {}", name)
}

#[test]
fn shadowing() {
    let name = "joko";
    println!("Hello {}", name);

    let name = "budiono";
    println!("Hello {}", name)
}

// terdapat beberapa tipe data untuk integer
// 8, 16, 32, 64, 128
// ukuran default bilangan bulat i32 dan desimal f64
#[test]
fn explicit() {
    let age: i32 = 15;
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
    let hasil = angka1 * angka2;

    print!("{}", hasil)
}

#[test]
fn augmented_assingment() {
    let mut angka1 = 10;
    let angka2 = 12;

    angka1 += 30;
    // angka1-=30;
    // angka1*=30;

    print!("{} {}", angka1, angka2)
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison_operation() {
    let a = 10;
    let b = 20;

    let result: bool = a > b;

    /*
    ada && yang berarti "dan"
    ada || yang berarti "atau"
    ada ! yang berarti "tidak/kebalikan"
     */
    println!("{}", result);
}
#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai_akhir = 80;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir: bool = nilai_akhir >= 75;

    let lulus: bool = lulus_absen && lulus_nilai_akhir;

    println!("Kamu: {}", lulus);
}

#[test]
fn char_type() {
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}

#[test]
fn tupple() {
    // let data: (i32, f64, bool) = (10, 10.5, true);
    let mut data: (i32, f64, bool) = (10, 10.5, true);

    println!("{:?}", data);

    // let a = data.0;
    // let b = data.1;
    // let c = data.2;

    let (a, b, c) = data;

    // mutable tupple
    data.0 = 20;
    data.1 = 20.5;
    data.2 = true;
    println!("{} {} {}", a, b, c)
}

// If-Else
#[test]
fn if_else_example() {
    let nilai = 70;
    let hasil_akhir: &str;
    
    if nilai >= 80 {
        hasil_akhir = "Nilai A";
    } else if nilai >= 70 {
        hasil_akhir = "Nilai B";
    } else if nilai >= 60 {
        hasil_akhir = "Nilai C";
    } else {
        hasil_akhir = "Nilai D";
    }
    println!("Hasil akhir: {}", hasil_akhir);
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
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", array);

    let length: usize = array.len();
    println!("{}", length);
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

#[test]
fn array_two_deminsional() {
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];

    println!("{:?}", matrix);

    println!("{}", matrix[0][0]);
    println!("{}", matrix[0][1]);
    println!("{}", matrix[1][0]);
    println!("{}", matrix[1][1]);
}

pub const MAXIMUM: i32 = 100;

#[test]
fn constant() {
    // kurang lebih seperti variabel lokal dan publik
    const MINIMUM: i32 = 0;

    println!("Nilai Minimum: {}", MINIMUM);
    println!("Nilai Maksimum: {}", MAXIMUM);
}

#[test]
fn variable_scope() {
    let uang_john = 1000;

    {
        println!("Uang John: {}", uang_john);
        let uang_santi = 2000;
        println!("Uang Santi: {}", uang_santi);
    }
    // tidak bisa akses uang santi atau "error"
    // println!("Uang Santi: {}", uang_santi)
}

#[test]
fn stack_heap() {
    function_a();
    function_b();

    // stack & heap
    // stack ini akan menghapus variable ketika function selesai
    // heap ini akan menghapus variable ketika program selesai
    // stack akan menghapus dari function b terlebih dahulu, kemudian function a
}

fn function_a() {
    let a = 10;
    let b = String::from("Hello");
    println!("{} {}", a, b);
}

fn function_b() {
    let a = 20;
    let b = String::from("World");
    println!("{} {}", a, b);
}


#[test]
fn string_type() {
    // terdapat beberapa macam operasi pada string
    // sumber https://doc.rust-lang.org/std/primitive.str.html
    let mut name = String::from("   Joko Sudirman Wibowo    ");
    println!("Hello {}", name);

    name.push_str(" Budiono");
    println!("Hello {}", name);

    name.push('!');
    println!("Hello {}", name);

    let length = name.len();
    println!("Panjang string: {}", length);

    for word in name.split_whitespace() {
        println!("{}", word);
    }

    let trim: &str = name.trim();
    println!("Trim: {}", trim);

    let replace = name.replace("Joko", "Santi");
    println!("Replace: {}", replace);

    let to_uppercase = name.to_uppercase();
    println!("Uppercase: {}", to_uppercase);

    let to_lowercase = name.to_lowercase();
    println!("Lowercase: {}", to_lowercase);

    let contains = name.contains("Joko");
    println!("Contains 'Joko': {}", contains);

    let substring = &name[0..4];
    println!("Substring: {}", substring);
}

#[test]
fn ownership_rules() {
    // a tidak bisa diakses disini belum dideklarasikan
    let a = 10;
    
    { // b tidak bisa diakses disini, karna belum dideklarasikan
        let b = a; // menyalin nilai a ke b, karena tipe data i32 adalah tipe data yang disalin (Copy)
        println!("Nilai b: {}", b);
    } // scope b slesai, b dihapus kemudian, b sudah tidak bisa diakses lagi

    println!("Nilai a: {}", a); // a masih bisa diakses di sini
} // scope a selesai, a dihapus kemudian, a sudah tidak bisa diakses lagi

#[test]
fn ownership() {
    let a = String::from("Hello");
    let b = a; // memindahkan ownership dari a ke b

    // println!("{}", a); // error karena a sudah tidak memiliki ownership
    println!("{}", b); // ini yang benar
}

#[test]
fn ownership_clone() {
    let a = String::from("Hello");
    let b = a.clone(); // menggandakan nilai a ke b

    println!("a = {}, b = {}", a, b);

    let angka1 = 10;
    let angka2 = angka1; // menyalin nilai angka1 ke angka2

    println!("angka1 = {}, angka2 = {}", angka1, angka2); 
}
fn takes_ownership(s: String) {
    println!("{}", s);
}

#[test]
fn ownership_move() {
    let a = String::from("Hello");
    takes_ownership(a); // memindahkan ownership ke fungsi

    // println!("{}", a); // error karena a sudah tidak memiliki ownership
}
