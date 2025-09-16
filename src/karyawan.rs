pub(crate) struct Karyawan {
    nama: String,
    salary: f64, // Jadikan variabel ini private
}

impl Karyawan {
    pub fn new(nama: String, salary: f64) -> Karyawan {
        Karyawan { nama, salary }
    }

    // Metode publik untuk mendapatkan gaji (getter)
    pub fn get_salary(&self) -> f64 {
        self.salary
    }

    pub fn get_name(&self) -> &str {
        &self.nama
    }

    // Metode publik untuk mengubah gaji (setter) dengan validasi
    pub fn set_salary(&mut self, new_salary: f64) {
        if new_salary > 0.0 {
            self.salary = new_salary;
        } else {
            println!("Gaji tidak bisa negatif!");
        }
    }
}