use axum::extract::Path;
use axum::http::StatusCode;

use axum::{Extension, Json};
use serde_json::{Value, json};

use crate::GenericPool;
use crate::errors::ApiError;

use super::model::{self, Task};


pub async fn all_tasks(Extension(pool): Extension<GenericPool>) -> Result<Json<Vec<Task>>, ApiError> {
    let sql = "SELECT * FROM task ".to_string();

    let task = sqlx::query_as::<_, model::Task>(&sql).fetch_all(&pool)
        .await
        .map_err(|_| {
            ApiError::InternalServerError
        })?;

    Ok(Json(task))
}

pub async fn task(
    Path(id):Path<i32>, 
    Extension(pool): Extension<GenericPool>) -> Result<Json<model::Task>, ApiError> {
        let task: model::Task = sqlx::query_as("SELECT * FROM task where id=?")
            .bind(id)
            .fetch_one(&pool)
            .await
            .map_err(|_| {
                ApiError::NotFound
            })?;

        Ok(Json(task))  
}

pub async fn new_task(
    Extension(pool): Extension<GenericPool>, 
    Json(task): Json<model::NewTask>) -> Result<(StatusCode, Json<model::Task>), ApiError> {
        if task.task.is_empty() {
            return Err(ApiError::BadRequest);
        }

        let id = sqlx::query("INSERT INTO task (task) values (?)")
                .bind(&task.task)
                .execute(&pool)
                .await
                .map_err(|_| {
                    ApiError::InternalServerError
                })?.last_insert_rowid();

        let task = model::Task { 
            id, 
            task: task.task
        };

        Ok((StatusCode::CREATED, Json(task)))
}

async fn find_task(pool: &GenericPool, id: i64) -> Result<(), ApiError> {
    let _find: model::Task = sqlx::query_as("SELECT * FROM task where id=?")
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(|_| {
            ApiError::NotFound
        })?;

        Ok(())
}

pub async fn update_task(
    Path(id): Path<i64>, 
    Extension(pool): Extension<GenericPool>,
    Json(task): Json<model::UpdateTask>) -> Result<(StatusCode, Json<model::Task>), ApiError> {
        find_task(&pool, id).await?;

        let _result = sqlx::query("UPDATE task SET task=? WHERE id=?")
            .bind(&task.task)
            .bind(id)
            .execute(&pool)
            .await;

        let task = model::Task {
            id,
            task: task.task
        };

        Ok((StatusCode::OK, Json(task)))
}

pub async fn delete_task(
    Path(id): Path<i64>, 
    Extension(pool): Extension<GenericPool>) -> Result<(StatusCode, Json<Value>), ApiError> {
        find_task(&pool, id).await?;

        sqlx::query("DELETE FROM task WHERE id=?")
            .bind(id)
            .execute(&pool)
            .await
            .map_err(|_| {
                ApiError::NotFound
            })?;

            Ok((StatusCode::OK, Json(json!({"msg": "Task Deleted"}))))
}
