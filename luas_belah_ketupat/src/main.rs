use std::io::{ self, Write };

fn luas_belah_ketupat(d1: f64, d2: f64) -> f64 {
    0.5 * d1 * d2
}

fn main() {
    println!("Program Luas Belah Ketupat");
    println!("==========================");

    print!("Input panjang diagonal 1: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_diagonal1 = String::new();
    io::stdin()
        .read_line(&mut input_diagonal1)
        .expect("Gagal membaca input!");

    let diagonal1: f64 = match input_diagonal1.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukkan angka yang valid!");
            return;
        }
    };

    print!("Input panjang diagonal 2: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_diagonal2 = String::new();
    io::stdin()
        .read_line(&mut input_diagonal2)
        .expect("Gagal membaca input!");

    let diagonal2: f64 = match input_diagonal2.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukkan angka yang valid!");
            return;
        }
    };

    println!("Luas belah ketupat: {}", luas_belah_ketupat(diagonal1, diagonal2));
}
