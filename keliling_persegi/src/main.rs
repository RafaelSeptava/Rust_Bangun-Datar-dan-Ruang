use std::io::{ self, Write };

fn keliling_persegi(s: f64) -> f64 {
    4.0 * s
}

fn main() {
    println!("Program Keliling Persegi");
    println!("========================");

    print!("Input sisi persegi: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_sisi = String::new();
    io::stdin()
        .read_line(&mut input_sisi)
        .expect("Gagal membaca input!");

    let sisi: f64 = match input_sisi.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukkan angka yang valid!");
            return;
        }
    };

    println!("Keliling persegi: {}", keliling_persegi(sisi));
}
