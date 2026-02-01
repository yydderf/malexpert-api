#[derive(Default)]
pub(crate) struct SSEFrame {
    pub(crate) id: Option<String>,
    pub(crate) event: Option<String>,
    pub(crate) data: String,
}


