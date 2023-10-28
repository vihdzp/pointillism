pub trait Coefficients {
    /// Gets the coefficient corresponding to a given term x<sup>e</sup>.
    fn get(exp: usize) -> f64;
}

struct DenseStc<const N: usize>([f64; N]);
