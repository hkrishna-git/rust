use num_traits::{Float, ToPrimitive};

fn solve1(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn solve2<T:Float>(a: T, b: T) -> T {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    println!("{}", solve1(3.0, 3.0));
    println!("{}", solve1(3 as f64, 3.0));
    println!("{}", solve1(3.0, 3.to_f64().unwrap()));

    println!("{}", solve2::<f32>(3.0, 3.0));
}
