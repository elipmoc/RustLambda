//SKIコンビネータのAST
#[derive(Debug, Clone)]
pub enum SkiAST {
    S,
    K,
    I,
    Apply(Box<SkiAST>, Box<SkiAST>),
}
