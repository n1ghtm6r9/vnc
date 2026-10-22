pub(crate) fn bessel_i0(x: f64) -> f64 {
    let quarter_square = x * x / 4.0;
    let mut term = 1.0;
    let mut sum = 1.0;
    let mut order = 1.0;
    while term > sum * 1e-12 {
        term *= quarter_square / (order * order);
        sum += term;
        order += 1.0;
    }
    sum
}
