use async_trait::async_trait;
use sqlx::{PgPool, types::Uuid};

use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse the `question_uuid` field
        // in `Answer` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = Uuid::parse_str(&answer.question_uuid)
            .map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        // Make a database query to insert a new answer.
        // Here is the SQL query:
        // ```
        // INSERT INTO answers ( question_uuid, content )
        // VALUES ( $1, $2 )
        // RETURNING *
        // ```
        // If executing the query results in an error, check to see if
        // the error code matches `postgres_error_codes::FOREIGN_KEY_VIOLATION`.
        // If so early return the `DBError::InvalidUUID` error. Otherwise early return
        // the `DBError::Other` error.
        let record = sqlx::query!(
            "INSERT INTO answers (question_uuid, content) VALUES ($1, $2) RETURNING *",
            uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e| {
            if let Some(code) = e.as_database_error().and_then(|db_err| db_err.code()) {
                if code == postgres_error_codes::FOREIGN_KEY_VIOLATION {
                    return DBError::InvalidUUID("Question not found".to_string());
                }
            }
            DBError::Other(e.into())
        })?;

        // Populate the AnswerDetail fields using `record`.
        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `answer_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid =
            Uuid::parse_str(&answer_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        // Make a database query to delete an answer given the answer uuid.
        // Here is the SQL query:
        // ```
        // DELETE FROM answers WHERE answer_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.
        sqlx::query!("DELETE FROM answers WHERE answer_uuid = $1", uuid)
            .execute(&self.db)
            .await
            .map_err(|e| DBError::Other(e.into()))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `question_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid =
            Uuid::parse_str(&question_uuid).map_err(|e| DBError::InvalidUUID(e.to_string()))?;

        // Make a database query to get all answers associated with a question uuid.
        // Here is the SQL query:
        // ```
        // SELECT * FROM answers WHERE question_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.
        let records = sqlx::query!("SELECT * FROM answers WHERE question_uuid = $1", uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(e.into()))?;

        // Iterate over `records` and map each record to a `AnswerDetail` type
        let answers = records
            .into_iter()
            .map(|record| AnswerDetail {
                answer_uuid: record.answer_uuid.to_string(),
                question_uuid: record.question_uuid.to_string(),
                content: record.content,
                created_at: record.created_at.to_string(),
            })
            .collect();

        Ok(answers)
    }
}
