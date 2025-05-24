use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// use aws_sdk_sesv2::types::Body;
// #[serde(remote = "Body")]
#[serde(rename_all = "PascalCase")]
pub struct Body {
    /// <p>An object that represents the version of the message that is displayed in email clients that don't support HTML, or clients where the recipient has disabled HTML rendering.</p>
    pub text: ::std::option::Option<crate::types::Content>,
    /// <p>An object that represents the version of the message that is displayed in email clients that support HTML. HTML messages can include formatted text, hyperlinks, images, and more.</p>
    pub html: ::std::option::Option<crate::types::Content>,
}
