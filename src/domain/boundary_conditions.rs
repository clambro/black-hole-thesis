#[derive(Clone, Debug, clap::ValueEnum, PartialEq)]
pub enum BoundaryCondition {
    Neumann,
    Dirichlet,
}

#[derive(Clone, Debug)]
pub struct BoundaryConditions {
    pub left: BoundaryCondition,
    pub right: BoundaryCondition,
}
