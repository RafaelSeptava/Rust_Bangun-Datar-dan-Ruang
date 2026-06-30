use std::io::{ self, Write };

fn luas_permukaan_balok(p: f64, l: f64, t: f64) -> f64 {
    2.0 * (p * l) + 2.0 * (p * t) + 2.0 * (l * t)
}

fn volume_balok(p: f64, l: f64, t: f64) -> f64 {
    p * l * t
}

fn main() {
    println!("Program Luas Permukaan dan Volume Balok");
    println!("=======================================");

    print!("Input panjang balok: ");
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

    print!("Input lebar balok: ");
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

    print!("Input tinggi balok: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_tinggi = String::new();
    io::stdin()
        .read_line(&mut input_tinggi)
        .expect("Gagal membaca input!");

    let tinggi: f64 = match input_tinggi.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukkan angka yang valid!");
            return;
        }
    };

    println!("Luas permukaan balok: {}", luas_permukaan_balok(panjang, lebar, tinggi));
    println!("Volume balok: {}", volume_balok(panjang, lebar, tinggi));
}
