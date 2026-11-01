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

