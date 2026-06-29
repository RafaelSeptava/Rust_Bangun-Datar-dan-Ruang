use std::io::{ self, Write };

fn keliling_persegi_panjang(p: f64, l: f64) -> f64 {
    (2.0 * p) + (2.0 * l)
}

fn main() {
    println!("Program Keliling Persegi Panjang");
    println!("================================");

    print!("Input panjang persegi: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_panjang = String::new();
    io::stdin()
        .read_line(&mut input_panjang)
        .expect("Gagal membaca input!");

    let panjang: f64 = match input_panjang.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukkan angka yang valid!");
            return;
        }
    };

    print!("Input lebar persegi: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_lebar = String::new();
    io::stdin()
        .read_line(&mut input_lebar)
        .expect("Gagal membaca input!");

    let lebar: f64 = match input_lebar.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukkan angka yang valid!");
            return;
        }
    };

    println!("Keliling persegi panjang: {}", keliling_persegi_panjang(panjang, lebar));
}
