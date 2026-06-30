use std::io::{ self, Write };

fn luas_permukaan_kubus(s: f64) -> f64 {
    6.0 * (s * s)
}

fn volume_kubus(s: f64) -> f64 {
    s * s * s
}

fn main() {
    println!("Program Luas Permukaan dan Volume Kubus");
    println!("=======================================");

    print!("Input panjang sisi kubus: ");
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

    println!("Luas permukaan kubus: {}", luas_permukaan_kubus(sisi));
    println!("Volume kubus: {}", volume_kubus(sisi))
}
