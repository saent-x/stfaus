use crate::models::SearchAgent;
use async_trait::async_trait;

pub struct AppleMusic;

#[async_trait]
impl SearchAgent for AppleMusic {
    async fn search(&self, _search_items: &[crate::models::LLMResult]) -> Result<Vec<crate::models::Song>, crate::models::AppError> {
        todo!()
    }
}
