use aws_sdk_sesv2::operation::send_email::SendEmailInput;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, std::fmt::Debug)]
#[serde(remote = "SendEmailInput")]
pub struct Request {}
