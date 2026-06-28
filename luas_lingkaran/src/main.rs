use std::io::{ self, Write };

fn luas_lingkaran(r: f64) -> f64 {
    3.14 * r * r
}

fn main() {
    println!("Program Luas Lingkaran");
    println!("======================");

    print!("Input jari-jari lingkaran: ");
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

    println!("Luas lingkaran: {}", luas_lingkaran(jari_jari));
}
