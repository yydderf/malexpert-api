#[macro_export]

macro_rules! api_error_ctor {
    ($name:ident, $status:expr) => {
        pub fn $name(title: impl Into<String>, detail: impl Into<String>) -> Self {
            Self {
                status: $status,
                body: APIBody::Err(APIErrorBody{ title: title.into(), detail: Some(detail.into()) }),
            }
        }
    }
}
