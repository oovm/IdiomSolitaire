pub use crate::theme::QRArtTheme;

mod theme;





pub struct QRArt {
    traverse: Traverse,
    background: bool,
    theme: QRArtTheme
}