use std::backtrace;

fn main() {
    println!("Hello, world!");
}

// Function unit tes

/*ini komentar lebih
dari satu baris*/

#[test]
fn hello_test(){
    println!("Hello Test");
}

#[test]
fn test_variable(){
    let name = "Insun";
    println!("Hello {}", name);
}

#[test]
fn test_variable_mutable(){
    let mut name = "Insun";
    println!("Hello {}", name);
    
    name = "Insun97";
    println!("Hello {}", name);

}

// #[test]
// fn static_typing(){
//     let mut name = "Insun";
//     println!("Hello {}", name);
    
//     // name = 97;
//     println!("Hello {}", name);

// }

#[test]
fn shadowing(){
    let name = "Insun";
    println!("Hello {}", name);
    
    let name = 97;
    println!("Hello {}", name);

}

#[test]
fn explicit(){
    let age = 1;
    println!("{}", age);
}

#[test]
fn number(){
    let a: i32 = 1;
    println!("{}", a);
    
    let b: f64 = 1.3;
    println!("{}", b);
}

#[test]
fn konversion(){
    let a: i8 = 1;
    println!("{}", a);
    
    let b: i16 = a as i16;
    println!("{}", b);

    // ketika ada konversi dibawah spesifikasi maka akan menghasilkan nilai 0
    let c: i64 = 100000000000000;
    let d: i8 = c as i8;
    println!("{}", d);
}

#[test]
fn numeric_operation(){
    let a = 1;
    let b = 1;
    let c = a + b;
    println!("{}", c);
}

#[test]
fn augmented_assigmnent(){
    let mut num = 1;
    println!("{}", num);

    num += 10;

    println!("{}", num);
}

#[test]
fn boolean(){
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparation_operator(){
    let a = 100;
    let b = 30;
    let result = a < b;

    println!("{} {} {}", a, b, result);
}

#[test]
fn boolean_operator(){
    let a = true;
    let b = true;
    
    // AND Operation
    let result_and = a && b;

    // OR Operation
    let result_or = a || b;

    let nilai = 75;
    let batas = 80;

    let nilai_akhir = nilai >= batas; 
    
    let lulus = !nilai_akhir && true;

    // OR Operation
    // let result_not = a ! b;

    println!("{} {} {}", result_and, result_or, lulus);
}

// jika tipe data ini digunakan lebih dari satu maka tidak akan bisa
#[test]
fn char_type(){
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {} ", char1, char2);
}

// seperti array tapi berbeda type data
#[test]
fn tuple(){
    let data: (i32, f64, bool) = (12, 10.5, true);

    println!("{:?} ",data);

// memanggil data berdasarkan urutan dimulai dari 0, seperti array pengggunaannnya
    // let a = data.0;
    // println!("Data ke {}", a);

// defraktor tuple
    // let (b, c, _) = data;
    // println!("{} {} {}", b, c, a);

// mutable data
data.0 = 10;
data.1 = 3.14;
data.2 = false;

println!("{:?} ",data);
// menit 1:31:35

}