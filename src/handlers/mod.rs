use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sqlx::types::Uuid;

use crate::{AppState, models::*};

mod handlers_inner;

impl IntoResponse for handlers_inner::HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            handlers_inner::HandlerError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
            handlers_inner::HandlerError::InternalError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
        }
    }
}

pub async fn create_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> Result<Json<QuestionDetail>, handlers_inner::HandlerError> {
    if question.title.is_empty() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Title is required".to_string(),
        ));
    }
    if question.description.is_empty() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Description is required".to_string(),
        ));
    }
    let question_detail = questions_dao.create_question(question).await;
    match question_detail {
        Ok(question_detail) => Ok(Json(QuestionDetail {
            question_uuid: question_detail.question_uuid,
            title: question_detail.title,
            description: question_detail.description,
            created_at: question_detail.created_at,
        })),
        Err(e) => Err(handlers_inner::HandlerError::InternalError(e.to_string())),
    }
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> Result<Json<Vec<QuestionDetail>>, handlers_inner::HandlerError> {
    let questions_detail = questions_dao.get_questions().await;
    match questions_detail {
        Ok(questions_detail) => Ok(Json(questions_detail)),
        Err(e) => Err(handlers_inner::HandlerError::InternalError(e.to_string())),
    }
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<(), handlers_inner::HandlerError> {
    if question_uuid.question_uuid.is_empty() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Question UUID is required".to_string(),
        ));
    }
    if !Uuid::parse_str(&question_uuid.question_uuid).is_ok() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Invalid question UUID".to_string(),
        ));
    }
    let result = questions_dao
        .delete_question(question_uuid.question_uuid)
        .await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(handlers_inner::HandlerError::InternalError(e.to_string())),
    }
}

pub async fn create_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> Result<Json<AnswerDetail>, handlers_inner::HandlerError> {
    if answer.content.is_empty() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Content is required".to_string(),
        ));
    }
    if !Uuid::parse_str(&answer.question_uuid).is_ok() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Invalid question UUID".to_string(),
        ));
    }
    let answer_detail = answers_dao.create_answer(answer).await;
    match answer_detail {
        Ok(answer_detail) => Ok(Json(answer_detail)),
        Err(e) => Err(handlers_inner::HandlerError::InternalError(e.to_string())),
    }
}

pub async fn read_answers(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> Result<Json<Vec<AnswerDetail>>, handlers_inner::HandlerError> {
    if question_uuid.question_uuid.is_empty() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Question UUID is required".to_string(),
        ));
    }
    if !Uuid::parse_str(&question_uuid.question_uuid).is_ok() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Invalid question UUID".to_string(),
        ));
    }
    let answer_detail = answers_dao.get_answers(question_uuid.question_uuid).await;
    match answer_detail {
        Ok(answer_detail) => Ok(Json(answer_detail)),
        Err(e) => Err(handlers_inner::HandlerError::InternalError(e.to_string())),
    }
}

pub async fn delete_answer(
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> Result<(), handlers_inner::HandlerError> {
    if answer_uuid.answer_uuid.is_empty() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Answer UUID is required".to_string(),
        ));
    }
    if !Uuid::parse_str(&answer_uuid.answer_uuid).is_ok() {
        return Err(handlers_inner::HandlerError::BadRequest(
            "Invalid answer UUID".to_string(),
        ));
    }
    let result = answers_dao.delete_answer(answer_uuid.answer_uuid).await;
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(handlers_inner::HandlerError::InternalError(e.to_string())),
    }
}
