use std::io::{ self, Write };

fn luas_permukaan_bola(r: f64) -> f64 {
    (4.0 / 3.0) * (22.0 / 7.0) * r * r * r
}

fn volume_bola(r: f64) -> f64 {
    4.0 * (22.0 / 7.0) * r * r
}

fn main() {
    println!("Program Luas Permukaan dan Volume Bola");
    println!("======================================");

    print!("Input jari-jari bola: ");
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

    println!("Luas permukaan bola: {}", luas_permukaan_bola(jari_jari));
    println!("Volume bola: {}", volume_bola(jari_jari));
}
