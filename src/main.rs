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
fn for_loop_label() {
    let mut count = 0;
    'outer: loop {
        println!("Perulangan luar ke-{}", count);
        let mut inner_count = 0;

        loop {
            println!("  Perulangan dalam ke-{}", inner_count);
            if inner_count == 2 {
                break; // Hanya keluar dari loop dalam
            }
            if count == 1 {
                break 'outer; // Keluar dari loop luar
            }
            inner_count += 1;
        }
        count += 1;
    }
}

#[test]
fn loop_array() {
    let array = [10, 20, 30, 40, 50];
    // let mut index = 0;
    let range_eklusif = 0..array.len();
    let range_inklusif = 0..=array.len()-1;

    // * loop biasa
    // loop {
    //     if index >= array.len() {
    //         break;
    //     }
    //     println!("Nilai array: {}", array[index]);
    //     index += 1;
    // }

    // * while loop
    // while index < array.len() {
    //     println!("Nilai array: {}", array[index]);
    //     index += 1;
    // }

    // * for loop
    // for element in array.iter() {
    //     println!("Nilai array: {}", element);
    // }

    // * for loop dengan range ekslusif
    for i in range_eklusif {
        println!("Nilai array: {}", array[i]);
    }

    // * for loop dengan range inklusif
    for i in range_inklusif {
        println!("Nilai array: {}", array[i]);
    }
}

#[test]
fn range() {
    let range_eklusif = 0..5; // 0, 1, 2, 3, 4
    println!("Start: {}", range_eklusif.start);
    println!("End: {}", range_eklusif.end);

    let array: [&str; 5] = ["A", "B", "C", "D", "E"];

    for i in range_eklusif {
        println!("Index ke-{} adalah {}", i, array[i]);
    }

    let range_inklusif = 0..=5; // 0, 1, 2, 3, 4, 5
    println!("Start: {}", range_inklusif.start());
    println!("End: {}", range_inklusif.end());

    for i in range_inklusif {
        println!("Index ke-{} adalah {}", i, array[i]);
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

pub fn function_a() {
    let a = 10;
    let b = String::from("Hello");
    println!("{} {}", a, b);
}

pub fn function_b() {
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
pub fn takes_ownership(s: String) {
    println!("{}", s);
}

#[test]
fn ownership_move() {
    let a = String::from("Hello");
    takes_ownership(a); // memindahkan ownership ke fungsi

    // println!("{}", a); // error karena a sudah tidak memiliki ownership
}

#[test]
fn test_function_parameter() {
    let nilai1 = 10;
    let nilai2 = 20;

    let hasil = tambah(nilai1, nilai2);
    println!("Hasil: {}", hasil);

    // a dan b masih bisa diakses disini karena tipe data i32 adalah tipe data yang disalin (Copy)
    println!("a: {}, b: {}", nilai1, nilai2);
}

pub fn tambah(a: i32, b: i32) -> i32 {
    // tanda panah itu merupakan tipe data yang dikembalikan oleh fungsi
    a + b
}

#[test]
fn recrusive_function() {
    let result = factorial(5);
    println!("Factorial: {}", result); // Factorial: 120
}

pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[test]
fn function_ownership() {
    let angka = 10;
    print_number(angka); // angka masih bisa digunakan setelah dipanggil
    println!("Angka di main: {}", angka);

    let nama = String::from("Joko");
    print_name(nama); // nama tidak bisa digunakan setelah dipanggil
    // println!("Nama di main: {}", nama); // error karena nama sudah tidak memiliki ownership
}

pub fn print_number(num: i32) {
    println!("Angka: {}", num);
}
pub fn print_name(name: String) {
    println!("Nama: {}", name);
}

#[test]
fn test_fullname() {
    let first_name = String::from("Joko");
    let last_name = String::from("Sudirman");

    let name = full_name(&first_name, &last_name);

    println!("Full Name: {}", name);
    println!("First Name: {}", first_name); // Masih bisa diakses
    println!("Last Name: {}", last_name);   // Masih bisa diakses
}

pub fn full_name(first_name: &str, last_name: &str) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_fullname_return() {
    let first_name = String::from("Joko");
    let last_name: String = String::from("Sudirman");

    // let (first_name, last_name, full_name) = full_name_return(&first_name, &last_name);
    let full_name = full_name_return(&first_name, &last_name).2;
    
    println!("Full Name: {}", full_name);
    println!("First Name: {}", first_name); // Masih bisa diakses
    println!("Last Name: {}", last_name);   // Masih bisa diakses
}

pub fn full_name_return(first_name: &String, last_name: &String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name.clone(), last_name.clone(), full_name)
}

#[test]
fn test_borrowing_mutable() {
    let mut name = String::from("Joko");
    println!("Before: {}", name);
    change_name(&mut name);
    println!("After: {}", name);
}

pub fn change_name(name: &mut String) {
    name.push_str(" Budiono");
}

#[test]
fn test_borrowing_multiple() {
    let mut name = String::from("Joko");
    println!("Before: {}", name);
    {
        let name2 = &mut name;
        println!("Name2: {}", name2);
        change_name(name2);
    }
    {
        let name3 = &mut name;
        println!("Name3: {}", name3);
        change_name(name3);
    }
    println!("After: {}", name);
}

#[test]
fn test_borrowing_scope() {
    let mut name = String::from("Joko");
    println!("Before: {}", name);
    {
        let name2 = &mut name;
        change_name(name2);
    }
    println!("After: {}", name);
}

#[test]
fn test_borrowing_immutable() {
    let name = String::from("Joko");
    let name2 = &name;
    let name3 = &name;
    println!("Name2: {}", name2);
    println!("Name3: {}", name3);
    // change_name(name2); // error karena name2 adalah referensi immutable
    // change_name(name3); // error karena name3 adalah referensi immutable
    println!("Name: {}", name);
}

#[test]
fn test_dangling_reference() {
    let reference = dangle();
    println!("Reference: {}", reference);
}

pub fn dangle() -> String {
    let s = String::from("Hello");
    s
}

// solusi
pub fn no_dangle() -> String {
    let s = String::from("Hello");
    s // mengembalikan kepemilikan s ke pemanggil
}

#[test]
fn test_no_dangling_reference() {
    let reference = no_dangle();
    println!("Reference: {}", reference);
}

#[test]
fn test_borrowing_slice() {
    let name = String::from("Joko Sudirman");
    let first_word = first_word(&name);
    println!("First word: {}", first_word);
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[test]
fn test_borrowing_slice_array() {
    let array = [10, 20, 30, 40, 50];
    let slice = &array[1..4];
    println!("Slice: {:?}", slice);
    for &item in slice.iter() {
        println!("Item: {}", item);
    }
}

