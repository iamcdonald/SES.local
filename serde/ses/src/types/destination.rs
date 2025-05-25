use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
// use aws_sdk_sesv2::types::Destination;
// #[serde(remote = "Destination")]
#[serde(rename_all = "PascalCase")]
pub struct Destination {
    pub to_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>An array that contains the email addresses of the "CC" (carbon copy) recipients for the email.</p>
    pub cc_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>An array that contains the email addresses of the "BCC" (blind carbon copy) recipients for the email.</p>
    pub bcc_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
