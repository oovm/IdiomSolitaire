#[derive(Debug, Clone)]
pub enum Traverse {
    Random,
    Cycle,
    First,
}

#[derive(Debug, Clone)]
pub struct QRArtTheme {
    rotate: bool,
    des: String,
    desr: String,
    background: Option<QRArtBackground>,
    patterns: QRArtPatterns
}

#[derive(Debug, Clone)]
pub struct QRArtPatterns {

}

#[derive(Debug, Clone)]
pub struct GridPattern {}

#[derive(Debug, Clone)]
pub struct QRArtBackground {
    x: u32,
    y: u32,
    size: u32,
    image: u32,
}