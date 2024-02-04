use std::fmt;
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};
#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    OutOfBounds,
    QuestionNotFound,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "cannot parse parameter {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameters"),
            Error::OutOfBounds => write!(f, "Pagination parameters are out of bounds"),
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}
impl Reject for Error {}
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        let status = match error {
            Error::ParseError(_) => StatusCode::BAD_REQUEST,
            Error::MissingParameters => StatusCode::BAD_REQUEST,
            Error::OutOfBounds => StatusCode::RANGE_NOT_SATISFIABLE,
            Error::QuestionNotFound => StatusCode::NOT_FOUND,
        };
        Ok(warp::reply::with_status(error.to_string(), status))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
