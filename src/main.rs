use text_io::read;
use std::io;
use std::io::Write;

fn main() {
    print!("Yapacağın işlevi sayı olarak ilk gün ne kadar yapacaksın? ");
    io::stdout().flush().unwrap();
    let process: f32 = read!();

    print!("Yapacağın işlevi her gün yüzde kaç arttıracaksın? ");
    io::stdout().flush().unwrap();
    let mut multiply_percent: f32 = read!();
    multiply_percent = multiply_percent / (100 as f32);

    println!("Fonksiyon = f(x)=x+(x*{})", multiply_percent);

    println!("İlk gün yapılacak sayı = {:?}", process);
    println!("44. gün yapılacak sayı = {:?}", multiply(process, multiply_percent, 44));
    println!("77. gün yapılacak sayı = {:?}", multiply(process, multiply_percent, 77));
    println!("124. gün yapılacak sayı = {:?}", multiply(process, multiply_percent, 124));
    println!("199. gün yapılacak sayı = {:?}", multiply(process, multiply_percent, 199));
    println!("365. gün yapılacak sayı = {:?}", multiply(process, multiply_percent, 365));
    println!("730. gün yapılacak sayı = {:?}", multiply(process, multiply_percent, 730));

    println!("1 yılın sonunda bu işi toplam {:?} kez yapacaksın.", multiply_total(process, multiply_percent, 365));
    println!("2 yılın sonunda bu işi toplam {:?} kez yapacaksın.", multiply_total(process, multiply_percent, 730));
}

fn multiply (process: f32, percent: f32, deep: i32) -> f32 {
    if deep == 0 {
        return process;
    }

    return multiply(process + (process * percent), percent, deep - 1);
}

fn multiply_total (process: f32, percent: f32, deep: i32) -> f64 {
    let mut total: f64 = 0.0;

    for i in 0..deep {
        total += multiply(process + (process * percent), percent, i) as f64;
    }

    return total
}