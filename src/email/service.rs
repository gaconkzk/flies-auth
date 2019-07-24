// email_service.rs
use crate::errors::ServiceError;
use crate::models::Invitation;

use lettre::{EmailAddress, Transport, Envelope, SmtpClient};
use lettre_email::{Email, EmailBuilder, mime::TEXT_PLAIN};
use lettre::smtp::{response::Response, authentication::Credentials};
use std::env;
use log::{info, debug};

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {
  // let email = Email::builder()
  //   // Addresses can be specified by the tuple (email, alias)
  //   .to((invitation.email.clone(), "Firstname Lastname"))
  //   // ... or by an address only
  //   .from("noreply@localhost")
  //   .subject("Flies invitation")
  //   .text("You are invited.")
  //   // .attachment_from_file(Path::new("Cargo.toml"), None, &TEXT_PLAIN)
  //   .build()?;

  // // let mut mailer = SmtpClient::new_simple("smtp.gmail.com")?
  // //   .credentials(Credentials::new(env::var("GMAIL_USERNAME")?, env::var("GMAIL_PASSWORD")?)).transport();

  // // info!("Mailer created");

  // let mut mailer = SmtpClient::new_unencrypted_localhost()?.transport();
  // let result = mailer.send(email.into())?;
  // if result.is_positive() {
    debug!("Email {:?} sent.", invitation);
    Ok(())
  // } else {
  //   Err(ServiceError::EmailError("Sending error.".to_string()))
  // }
}
