use crate::models::*;
use axum::{Json, response::IntoResponse};

mod handlers_inner;

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    Json(QuestionDetail {
        question_uuid: "question_uuid".to_string(),
        title: question.title,
        description: question.description,
        created_at: "created_at".to_string(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    Json(Vec::<QuestionDetail>::new())
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {}

pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    Json(AnswerDetail {
        answer_uuid: "answer_uuid".to_string(),
        question_uuid: answer.question_uuid,
        content: answer.content,
        created_at: "created_at".to_string(),
    })
}

pub async fn read_answers(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
    Json(Vec::<AnswerDetail>::new())
}

pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) {}
