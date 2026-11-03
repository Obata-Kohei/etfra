use exmex::{BinOp, MakeOperators, Operator, ops_factory};
use num_complex::Complex;
use crate::util::types::Float;

ops_factory!(
    ComplexOpsFactory,
    Complex<Float>,

    // 定数
    Operator::make_constant("PI", Complex { re: 3.141592653589, im: 0.0 }),
    Operator::make_constant("E", Complex { re: 2.718281828459, im: 0.0 }),

    // 二項演算
    Operator::make_bin("+", BinOp {
        apply: |a, b| a + b,
        prio: 1,
        is_commutative: true
    }),
    Operator::make_bin("-", BinOp {
        apply: |a, b| a - b,
        prio: 1,
        is_commutative: false
    }),
    Operator::make_bin("*", BinOp {
        apply: |a, b| a * b,
        prio: 2,
        is_commutative: true
    }),
    Operator::make_bin("/", BinOp {
        apply: |a, b| a / b,
        prio: 2,
        is_commutative: false
    }),
    Operator::make_bin("^", BinOp {
        apply: |a, b| a.powc(b),
        prio: 3,
        is_commutative: false
    }),

    // 実部・虚部
    Operator::make_unary("re", |z| Complex::new(z.re, 0.0)),
    Operator::make_unary("im", |z| Complex::new(z.im, 0.0)),

    // 三角関数
    Operator::make_unary("sin", |z| z.sin()),
    Operator::make_unary("cos", |z| z.cos()),
    Operator::make_unary("tan", |z| z.tan()),

    // 双曲線関数
    Operator::make_unary("sinh", |z| z.sinh()),
    Operator::make_unary("cosh", |z| z.cosh()),
    Operator::make_unary("tanh", |z| z.tanh()),

    // 指数・対数
    Operator::make_unary("exp", |z| z.exp()),
    Operator::make_unary("log", |z| z.ln()),
    Operator::make_unary("log10", |z| z.log10()),

    // その他
    Operator::make_unary("sqrt", |z| z.sqrt()),
    Operator::make_unary("conj", |z| z.conj()),
    Operator::make_unary("abs", |z| Complex::new(z.norm(), 0.0))
);