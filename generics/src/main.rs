use num_traits::{Float, ToPrimitive};

fn solve1(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn solve2<T:Float>(a: T, b: T) -> T {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn solve3<T1: Float, T2: Float>(a : T1, b: T2) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve4<T1: ToPrimitive, T2: ToPrimitive>(a : T1, b: T2) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt() 
}

fn main() {
    println!("{}", solve1(3.0, 3.0));
    println!("{}", solve1(3 as f64, 3.0));
    println!("{}", solve1(3.0, 3.to_f64().unwrap()));

    println!("{}", solve2::<f32>(3.0, 3.0));
    println!("{}", solve2::<f64>(3.0, 3.0));

    println!("{}", solve3::<f32, f64>(3.0, 3.0));

    println!("{}", solve4::<i32, i64>(3, 3));
}
