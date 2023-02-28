use lin_alg_lib::*;

fn main() {
    let mut m1 = Matrix::new_templated(5, 2, 2);

    m1 *= 10000000;

    m1[(1, 0)] = 122;
    m1[(2, 0)] = 12;
    m1[(1, 1)] = 12;
    m1[(4, 1)] = 12;

    println!("{m1}");
    //println!("{}", m1.column(3));
}