pub trait Initialize {
    type output;
    // 初始化
    fn new() -> Self::output;
}