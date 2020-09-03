// Experiment with some basic scalar values
//      (I got lost; just check the class github for reference)

fn main() {
    // Integers
    println!("\n**** Integers");

    let i1 = 23;
    let mut i2 : u64 =2;
    for _ in 0..10 { // _ can be used if you aren't using index variable
        i2 *= 2;
    }

    println!("i1: {}:{}, i2 {}:{}", i1, type_of(&i1), i2, type_of(&i2));

    // Floats
    println!("\n**** Floats");

    let f1 = 123.45;
    let mut f2 = 2e-2;
    for i3 in 0..10 {
        f2 += i3 as f64;
    }

    let f3 = 123 as f32;

    println!("f1: {}:{}, f2 {}:{}, f3 {}:{}", f1, type_of(&f1), f2, type_of(&f2), f3, type_of(&f3));
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
