use crate::core::{
    real::Real,
    dynamics::Dynamics,
    escape_evaluator::*,
    escape_condition::EscapeCondition,
    normalize_esc_info::NormalizeEscInfo,
    color_map::ColorMap,
    coloring::Coloring,
};

pub struct EscapeTimeFractal<R, D, E, N, M>
where
    R: Real,
    D: Dynamics<Real = R>,
    E: EscapeEvaluator<D, Result = IterResult>,
    N: NormalizeEscInfo<E::Result>,
    M: ColorMap,
{
    plane: Plane<R>,
    dynamics: D,
    evaluator: E,
    coloring: Coloring<N, M>,
}
