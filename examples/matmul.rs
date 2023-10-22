use cognius::Tensor;

fn main() {
    // let a = Tensor::ones(vec![3, 2, 3, 2, 2, 3]);
    // let b = Tensor::from_f64(
    //     vec![
    //         1.9269, 1.4873, 0.9007, -2.1055, 0.6784, -1.2345, -0.0431, -1.6047, -0.7521, 1.6487,
    //         -0.3925, -1.4036, -0.7279, -0.5594, -0.7688, 0.7624, 1.6423, -0.1596, -0.4974, 0.4396,
    //         -0.7581, 1.0783, 0.8008, 1.6806, 1.2791, 1.2964, 0.6105, 1.3347, -0.2316, 0.0418,
    //         -0.2516, 0.8599, -1.3847, -0.8712, -0.2234, 1.7174, 0.3189, -0.4245, -0.8140, -0.7360,
    //         -0.8371, -0.9224, 1.8113, 0.1606, 0.3672, 0.1754, -1.1845, 1.3835, -1.2024, 0.7078,
    //         -1.0759, 0.5357, 1.1754, 0.5612,
    //     ],
    //     vec![3, 1, 3, 1, 2, 3],
    // );
    // let c = cognius::linalg::cross(a.clone(), b.clone());
    // println!("{}", c);

    let a = Tensor::ones(vec![3, 2, 3]);
    let b = Tensor::from_f64(
        vec![
            0.3367, 0.1288, 0.2345, 0.2303, -1.1229, -0.1863, 2.2082, -0.6380, 0.4617,
        ],
        vec![3, 1, 3],
    );
    let c = cognius::linalg::cross(a.clone(), b.clone());
    println!("{}", c);

    // let a = Tensor::from_f64(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2, 3]);
    // let b = a.t();
    // println!("{:?}", a.stride());
    // println!("{:?}", b.stride());

    // let a = Tensor::arange(1.0, 9.0, 1.0);
    // let a = a.reshape(&[2, 2, 2]);
    // let b = 5;
    // let c = a * b;
    // println!("{c}");

    // let a = Tensor::from_f64(
    //     vec![
    //         0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18.,
    //         19., 20., 21., 22., 23.,
    //     ],
    //     vec![1, 2, 3, 4],
    // );
    // println!("Data: {:?}\nShape: {:?}\nStride: {:?}", a.item(), a.shape, a.stride);
    // let a = Tensor::arange(0.0, 10.0, 1.0);
    // println!("Data: {:?}\nShape: {:?}\nStride: {:?}", a.item(), a.shape, a.stride);
    // let a = a.expand(&[2,2,3,4]);
    // println!("Data: {:?}\nShape: {:?}", a.item(), a.shape);
    // println!("{a}");

    // let a = a.transpose(1, 2);
    // println!("{a}");

    // let a = a.t();
    // println!("Data: {:?}\nShape: {:?}", a.item(), a.shape);
}
