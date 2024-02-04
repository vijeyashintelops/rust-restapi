use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};
use handle_errors::Error;
use std::collections::HashMap;
use warp::http::StatusCode;
use warp::{Rejection, Reply};
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        if pagination.start >= res.len() || pagination.end > res.len() {
            return Err(warp::reject::custom(Error::OutOfBounds));
        }
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res = store
            .questions
            .read()
            .await
            .values()
            .cloned()
            .collect::<Vec<Question>>();
        Ok(warp::reply::json(&res))
    }
}
pub async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question Added", StatusCode::OK))
}
pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question Updated", StatusCode::OK))
}
pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => {
            return Ok(warp::reply::with_status(
                "Questions Deleted",
                StatusCode::OK,
            ))
        }
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
