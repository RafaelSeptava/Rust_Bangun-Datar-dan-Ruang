use std::io::{ self, Write };

fn luas_permukaan_kerucut(r: f64, t: f64) -> f64 {
    let garis_pelukis = ((r * r) + (t * t)).sqrt();

    let luas_alas = (22.0 / 7.0) * r * garis_pelukis;
    let luas_selimut = (22.0 / 7.0) * r * r;

    luas_alas + luas_selimut
}

fn volume_kerucut(r: f64, t: f64) -> f64 {
    (1.0 / 3.0) * (22.0 / 7.0) * r * r * t
}

fn main() {
    println!("Program Luas Permukaan dan Volume Kerucut");
    println!("=========================================");

    print!("Input jari-jari kerucut: ");
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

    print!("Input tinggi kerucut: ");
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

    println!("Luas permukaan kerucut: {}", luas_permukaan_kerucut(jari_jari, tinggi));
    println!("Volume kerucut: {}", volume_kerucut(jari_jari, tinggi));
}
