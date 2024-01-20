use cognius::{
    module::Forward,
    nn::{functional as F, Linear},
    Tensor,
};

fn main() {
    let linear = Linear::new(20, 10);
    let x = Tensor::randn(&[2, 20]);
    println!("Weights: {}", linear.weight);
    println!("IN:\n{x}");
    let out = linear.forward(x);
    println!("OUT:\n{out}");
    let out = F::sigmoid(out);
    println!("OUT:\n{out}");
}
