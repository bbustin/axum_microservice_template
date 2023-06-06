use axum::extract::{Path, State};
use axum::http::StatusCode;

use axum::Json;
use serde_json::{json, Value};

use crate::errors::ApiError;
use crate::{AppState, GenericPool};

use super::model::Task;

#[utoipa::path(
    get,
    path = "/tasks",
    responses(
        (status = 200, description = "tasks", body = Vec<Task>)
    )
)]
pub async fn all_tasks(State(state): State<AppState>) -> Result<Json<Vec<Task>>, ApiError> {
    let sql = "SELECT * FROM tasks ".to_string();

    let task = sqlx::query_as::<_, Task>(&sql)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?;

    Ok(Json(task))
}

#[utoipa::path(
    get,
    path = "/tasks/{id}",
    responses(
        (status = 200, description = "task", body = Task)
    )
)]
pub async fn task(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Task>, ApiError> {
    let task: Task = sqlx::query_as("SELECT * FROM tasks where id=?")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| ApiError::NotFound)?;

    Ok(Json(task))
}

#[utoipa::path(
    post,
    path = "/tasks",
    responses(
        (status = 200, description = "task", body = Task)
    )
)]
pub async fn new_task(
    State(state): State<AppState>,
    Json(task): Json<Task>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    if task.id.is_some() || task.task.is_empty() {
        return Err(ApiError::BadRequest);
    }

    let id = sqlx::query("INSERT INTO tasks (task) values (?)")
        .bind(&task.task)
        .execute(&state.pool)
        .await
        .map_err(|_| ApiError::InternalServerError)?
        .last_insert_rowid();

    let task = Task {
        id: Some(id),
        task: task.task,
    };

    Ok((StatusCode::CREATED, Json(task)))
}

async fn find_task(pool: &GenericPool, id: i64) -> Result<(), ApiError> {
    let _find: Task = sqlx::query_as("SELECT * FROM tasks where id=?")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|_| ApiError::NotFound)?;

    Ok(())
}

#[utoipa::path(
    put,
    path = "/tasks/{id}",
    responses(
        (status = 200, description = "task", body = Task)
    )
)]
pub async fn update_task(
    Path(id): Path<i64>,
    State(state): State<AppState>,
    Json(task): Json<Task>,
) -> Result<(StatusCode, Json<Task>), ApiError> {
    if task.id.is_some() && task.id.unwrap() != id {
        return Err(ApiError::BadRequest);
    }

    find_task(&state.pool, id).await?;

    let _result = sqlx::query("UPDATE tasks SET task=? WHERE id=?")
        .bind(&task.task)
        .bind(id)
        .execute(&state.pool)
        .await;

    let task = Task {
        id: Some(id),
        task: task.task,
    };

    Ok((StatusCode::OK, Json(task)))
}

#[utoipa::path(delete, path = "/tasks/{id}")]
pub async fn delete_task(
    Path(id): Path<i64>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    find_task(&state.pool, id).await?;

    sqlx::query("DELETE FROM tasks WHERE id=?")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|_| ApiError::NotFound)?;

    Ok((StatusCode::OK, Json(json!({"msg": "Task Deleted"}))))
}
