use std::fmt::Debug;

use ses_serde::{operations::send_email::SendEmailInput, types::Destination};

mod raw_email;
mod simple_email;
mod template_email;
mod unknown_email;
pub use raw_email::RawEmail;
pub use simple_email::SimpleEmail;
pub use template_email::TemplateEmail;
pub use unknown_email::UnknownEmail;

pub trait EmailWrapper: Debug {
    fn get_subject(_: &SendEmailInput) -> Option<&String> {
        None
    }
    fn get_to(_: &SendEmailInput) -> Option<&Destination> {
        None
    }
    fn get_from(_: &SendEmailInput) -> Option<&String> {
        None
    }
    fn get_body(_: &SendEmailInput) -> Option<&String> {
        None
    }
}
