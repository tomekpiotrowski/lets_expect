#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Test,
    PubMethod,
    PubAsyncMethod,
    #[cfg(feature = "tokio")]
    TokioTest,
}
