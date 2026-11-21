use exmex::prelude::{Express, FlatEx};
use num_complex::Complex;
use crate::presets::custom_complex_dynamics::formula::ComplexOpsFactory;
use crate::{dynamics::Dynamics, util::types::Float};

type ComplexExpr = FlatEx<Complex<Float>, ComplexOpsFactory>;

pub struct CustomComplexDynamics {
    expr: ComplexExpr,
}

impl CustomComplexDynamics {
    pub fn new(expr: &str) -> Result<Self, exmex::ExError> {
        let expr = ComplexExpr::parse(expr)?;
        Ok(Self { expr })
    }
}

impl Dynamics for CustomComplexDynamics {
    type State = Complex<Float>;
    type Param = Complex<Float>;

    fn param_from_xy(
        &self,
        point: (Float, Float),
    ) -> Self::Param {
        Complex::new(point.0, point.1)
    }

    fn initial_state(&self, _p: &Self::Param) -> Self::State {
        Complex::ZERO
    }

    fn step(&self, z: &Self::State, c: &Self::Param) -> Self::State {
        let x = Complex::new(z.re, 0.0);
        let y = Complex::new(0.0, z.im);

        let vars: Vec<_> = self.expr.var_names().iter().map(|name| {
            match name.as_str() {
                "c" => *c,
                "x" => x,
                "y" => y,
                "z" => *z,
                _ => Complex::new(Float::NAN, Float::NAN),
            }
        }).collect();

        self.expr
            .eval(&vars)
            .unwrap_or(Complex::new(Float::NAN, Float::NAN))
    }
}