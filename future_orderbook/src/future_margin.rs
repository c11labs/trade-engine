use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum FutureMargin {
    Cross(f64),
    Isolated(f64),
}
