// errors.rs
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DBError};
use lettre::error::Error as EmailError;
use lettre_email::error::Error as EmailBuilderError;
use lettre::smtp::error::Error as SmtpEmailError;
use std::convert::From;
use std::env::VarError;
use uuid::parser::ParseError;

#[derive(Debug, Display)]
pub enum ServiceError {
  #[display(fmt = "Internal Server Error")]
  InternalServerError,
  #[display(fmt = "BadRequest: {}", _0)]
  BadRequest(String),
  #[display(fmt = "Unauthorized")]
  Unauthorized,
  // emails
  #[display(fmt = "EmailError: {}", _0)]
  EmailError(String),
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::EmailError(ref message) => HttpResponse::InternalServerError().json(message),
        }
    }
}

// we can return early in our handlers if UUID provided by the user is not valid
// and provide a custom message
impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<DBError> for ServiceError {
  fn from(error: DBError) -> ServiceError {
    // Right now we just care about UniqueViolation from diesel
    // But this would be helpful to easily map errors as our app grows
    match error {
      DBError::DatabaseError(kind, info) => {
          if let DatabaseErrorKind::UniqueViolation = kind {
              let message = info.details().unwrap_or_else(|| info.message()).to_string();
              return ServiceError::BadRequest(message);
          }
          ServiceError::InternalServerError
      }
      _ => ServiceError::InternalServerError,
    }
  }
}

impl From<EmailError> for ServiceError {
  fn from(error: EmailError) -> ServiceError {
    match error {
      EmailError::MissingFrom => ServiceError::EmailError("From address missing.".to_string()),
      EmailError::MissingTo => ServiceError::EmailError("To address missing.".to_string()),
      EmailError::InvalidEmailAddress => ServiceError::EmailError("Invalid email address.".to_string()),
    }
  }
}

impl From<SmtpEmailError> for ServiceError {
  fn from(error: SmtpEmailError) -> ServiceError {
    match error {
      _ => ServiceError::EmailError("Error sending email.".to_string())
    }
  }
}

impl From<EmailBuilderError> for ServiceError {
  fn from(error: EmailBuilderError) -> ServiceError {
    match error {
      _ => ServiceError::EmailError("Error create the email".to_string()),
    }
  }
}

impl From<VarError> for ServiceError {
  fn from(error: VarError) -> ServiceError {
    match error {
      _ => ServiceError::EmailError("Error getting email credentials from Environment".to_string()),
    }
  }
}
