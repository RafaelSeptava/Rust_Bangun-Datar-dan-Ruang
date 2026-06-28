use std::io::{ self, Write };

fn luas_trapesium(a: f64, b: f64, t: f64) -> f64 {
    0.5 * (a + b) * t
}

fn main() {
    println!("Program Luas Trapesium");
    println!("======================");

    print!("Input panjang sisi atas trapesium: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_sisi_atas = String::new();
    io::stdin()
        .read_line(&mut input_sisi_atas)
        .expect("Gagal membaca input!");

    let sisi_atas: f64 = match input_sisi_atas.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error! Masukkan angka yang valid!");
            return;
        }
    };

    print!("Input panjang sisi bawah trapesium: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_sisi_bawah = String::new();
    io::stdin()
        .read_line(&mut input_sisi_bawah)
        .expect("Gagal membaca input!");

    let sisi_bawah: f64 = match input_sisi_bawah.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error! Masukkan angka yang valid!");
            return;
        }
    };

    print!("Input tinggi trapesium: ");
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
            println!("Error! Masukkan angka yang valid!");
            return;
        }
    };

    println!("Luas Trapesium: {}", luas_trapesium(sisi_atas, sisi_bawah, tinggi));
}
