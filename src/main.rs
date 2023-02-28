use lin_alg_lib::*;

fn main() {
    let mut m1 = Matrix::new_templated(1, 4, 2);

    let m2 = Matrix::new_templated(4, 4, 8);

    let m_sum = &m1 + &m2;

    m1 -= &m2;

    println!("Sum:\n{}", m_sum);
    println!("M1:\n{}", m1);
}