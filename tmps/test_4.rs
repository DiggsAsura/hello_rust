fn main() {
    // testing float 32 vs 64
    let x: f32 = 0.2;
    let y: f32 = 0.1;
    let z: f32 = x + y;

    let a: f64 = 0.2;
    let b: f64 = 0.1;
    let c: f64 = a + b;

    println!("f64 (default): {a} + {b} = {c}");
    println!("f32 (specify); {x} + {y} = {z}");
}
