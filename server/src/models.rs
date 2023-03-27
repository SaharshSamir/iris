use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GoogleAuthCallbackQueryParams {
    pub code: String,
}
