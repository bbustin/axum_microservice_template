use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Deserialize, Serialize, ToSchema)]
pub struct Task {
    pub id: Option<i64>,
    pub task: String,
}