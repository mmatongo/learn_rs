use std::f64::consts;

fn main() {
    let x = 2.0 * consts::PI;
    let abs_differnce = (x.cos() - 1.0).abs();
    assert!(abs_differnce < 1e-10);
}
