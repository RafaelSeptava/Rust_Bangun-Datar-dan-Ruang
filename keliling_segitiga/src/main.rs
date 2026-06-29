use std::io::{ self, Write };

fn keliling_segitiga(a: f64, t: f64) -> f64 {
    a + t + ((a * a) + (t * t)).sqrt()
}

fn main() {
    println!("Program Keliling Segitiga");
    println!("=========================");

    print!("Input alas segitiga: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input_alas = String::new();
    io::stdin()
        .read_line(&mut input_alas)
        .expect("Gagal membaca input!");

    let alas: f64 = match input_alas.trim().parse() {
        Ok(angka)   => angka,
        Err(_)  => {
            println!("Error!!! Masukkan angka yang valid!");
            return;
        }
    };

    print!("Input tinggi segitiga: ");
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

    println!("Keliling segitiga: {}", keliling_segitiga(alas, tinggi));
}
