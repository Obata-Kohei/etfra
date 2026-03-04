use crate::core::state::State;
use crate::util::complex::Complex;
use crate::core::real::Real;

#[derive(Clone)]
pub struct MandelbrotState<R: Real> {
    pub z: Complex<R>,
    pub c: Complex<R>,
}

impl<R: Real> State for MandelbrotState<R> {
    type Real = R;

    fn new(point: (R, R)) -> Self {
        let c = Complex {
            re: point.0,
            im: point.1,
        };

        // z0 = 0
        let prec = 64; // 本来は外から渡す方がよい
        let z = Complex::zero(prec);

        Self { z, c }
    }
}