use std::cell::Ref;

use crate::{linalg, op::Op, tensor_data::TensorData, Tensor};

/// Backward trait for backpropagation operation.
pub trait Backward {
    fn backward(&self, tensor: &Ref<TensorData>);
}

/// `Backward` trait implemeted for `Op` where each operation has it's own
/// backpropagation operation.
impl Backward for Op {
    fn backward(&self, tensor: &Ref<TensorData>) {
        match self {
            // Addition backward
            // 1.0 * grad for both previous tensors
            Op::Add => {
                let t = tensor;
                let grad = t.grad.clone().unwrap();
                t._prev[0].add_to_grad(grad.clone());
                t._prev[1].add_to_grad(grad);
            }

            // Addition backward
            // 1.0 * grad for both previous tensors
            Op::Sub => {
                let t = tensor;
                let grad = t.grad.clone().unwrap();
                t._prev[0].add_to_grad(grad.clone());
                t._prev[1].add_to_grad(grad);
            }

            // Multiplication backward
            // a * b
            // da = b * grad
            // db = a * grad
            Op::Mul => {
                let t = tensor;
                let grad = t.grad.clone().unwrap();
                let l_item = t._prev[0].item();
                let r_item = t._prev[1].item();
                t._prev[0].add_to_grad(
                    r_item
                        .iter()
                        .zip(grad.clone())
                        .map(|(a, b)| a * b)
                        .collect(),
                );
                t._prev[1].add_to_grad(l_item.iter().zip(grad).map(|(a, b)| a * b).collect());
            }

            // Power backward
            // d(x^n)/dx * grad = n * x^(n-1) * grad
            Op::Pow(n) => {
                let n = *n;
                let t = tensor;
                let grad = t.grad.clone().unwrap();
                t._prev[0].add_to_grad(
                    t.data
                        .iter()
                        .zip(grad)
                        .map(|(x, g)| n as f64 * x.powi(n - 1) * g)
                        .collect(),
                );
            }

            // Exponent backward
            // d(e^x)/dx = e^x
            Op::Exp(t) => {
                let grad = tensor.grad.clone().unwrap();
                tensor._prev[0].add_to_grad(
                    t.exp()
                        .item()
                        .iter()
                        .zip(grad)
                        .map(|(a, b)| a * b)
                        .collect(),
                );
            }

            // Matrix Multiplication backward
            // da = dc @ b.T
            // db = a.T @ dc
            Op::MatMul => {
                let d_c = Tensor::new(TensorData::from_f64(
                    tensor.grad.clone().unwrap(),
                    tensor.shape.clone(),
                ))
                .requires_grad(false);
                let a = tensor._prev[0].t();
                let b = tensor._prev[1].t();
                tensor._prev[0].add_to_grad(linalg::matmul(d_c.clone(), b).item());
                tensor._prev[1].add_to_grad(linalg::matmul(a, d_c).item());
            }

            // Cross Section Multiplication backward
            Op::Cross => {
                todo!();
            }

            // ReLU backward
            // d(relu)/dx = { 1 if x > 0 else 0 }
            Op::ReLU => {
                let t = tensor;
                let mut prev = t._prev[0].inner.borrow_mut();
                let grad = prev.grad.as_mut().unwrap();
                for (i, g) in grad.iter_mut().enumerate() {
                    *g = if t.data[i] > 0.0 { 1.0 } else { 0.0 }
                }
            }

            // Sigmoid backward
            // sigmoid function:        1 / (1 + exp(-x))
            // dx(sigmoid) function:    exp(-x) / (1 + exp(-x))^2
            Op::Sigmoid(x) => {
                let e_x = (-x.clone()).exp();
                let res = e_x.clone() / (e_x + 1.0).pow(2);
                let grad = tensor.grad.clone().unwrap();
                let dx = grad.iter().zip(res.item()).map(|(a, b)| a * b).collect();
                tensor._prev[0].add_to_grad(dx);
            }

            Op::MSE => {
                let t = tensor._prev[0].inner.borrow();
                let t_sub = t._prev[0].inner.borrow();
                let out = t_sub._prev[0].item();
                let target = t_sub._prev[1].item();
                let grad = out
                    .iter()
                    .zip(target)
                    .map(|(x, y)| 2.0 * (x - y))
                    .collect::<Vec<f64>>();
                drop(t_sub);
                drop(t);
                tensor._prev[0].add_to_grad(grad);
            }
        }
    }
}
