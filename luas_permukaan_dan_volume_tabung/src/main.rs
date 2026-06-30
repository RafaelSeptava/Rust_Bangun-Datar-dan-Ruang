use std::io::{ self, Write };

fn luas_permukaan_tabung(r: f64, t: f64) -> f64 {
    2.0 * (22.0 / 7.0) * r * (r + t)
}

fn volume_tabung(r: f64, t: f64) -> f64 {
    (22.0 / 7.0) * t * r * r
}

fn main() {
    println!("Program Luas Permukaan dan Volume Tabung");
    println!("========================================");

    print!("Input jari-jari tabung: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_jari_jari = String::new();
    io::stdin()
        .read_line(&mut input_jari_jari)
        .expect("Gagal membaca input!");

    let jari_jari: f64 = match input_jari_jari.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukkan angka yang valid!");
            return;
        }
    };

    print!("Input tinggi tabung: ");
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

    println!("Luas permukaan tabung: {}", luas_permukaan_tabung(jari_jari, tinggi));
    println!("Volume tabung: {}", volume_tabung(jari_jari, tinggi));
}
