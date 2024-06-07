use std::io;

fn main() {
    println!("SigMATH!");

    println!("Masukkan bilangan yang akan menjadi sigma (sum):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Gagal membaca baris");

    // Normalisasi input menjadi bilangan bulat biasa
    let angka: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Ohio . input bilangan positif.");
            return;
        }
    };

    let nilai = sigma(angka);

    println!("Nilai sigma dari {} adalah: {}", angka, nilai);
}

fn sigma(n: u64) -> u64 {
    (1..=n).sum()
}
