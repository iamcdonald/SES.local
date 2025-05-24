// use aws_sdk_sesv2::operation::send_email::SendEmailInput;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(remote = "SendEmailInput")]
#[serde(rename_all = "PascalCase")]
pub struct SendEmailInput {
    /// <p>The email address to use as the "From" address for the email. The address that you specify has to be verified.</p>
    pub from_email_address: ::std::option::Option<::std::string::String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>FromEmailAddress</code> parameter.</p>
    /// <p>For example, if the owner of example.com (which has ARN arn:aws:ses:us-east-1:123456789012:identity/example.com) attaches a policy to it that authorizes you to use sender@example.com, then you would specify the <code>FromEmailAddressIdentityArn</code> to be arn:aws:ses:us-east-1:123456789012:identity/example.com, and the <code>FromEmailAddress</code> to be sender@example.com.</p>
    /// <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    /// <p>For Raw emails, the <code>FromEmailAddressIdentityArn</code> value overrides the X-SES-SOURCE-ARN and X-SES-FROM-ARN headers specified in raw email message content.</p>
    pub from_email_address_identity_arn: ::std::option::Option<::std::string::String>,
    /// <p>An object that contains the recipients of the email message.</p>
    pub destination: ::std::option::Option<crate::types::Destination>,
    /// <p>The "Reply-to" email addresses for the message. When the recipient replies to the message, each Reply-to address receives the reply.</p>
    pub reply_to_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The address that you want bounce and complaint notifications to be sent to.</p>
    pub feedback_forwarding_email_address: ::std::option::Option<::std::string::String>,
    /// <p>This parameter is used only for sending authorization. It is the ARN of the identity that is associated with the sending authorization policy that permits you to use the email address specified in the <code>FeedbackForwardingEmailAddress</code> parameter.</p>
    /// <p>For example, if the owner of example.com (which has ARN arn:aws:ses:us-east-1:123456789012:identity/example.com) attaches a policy to it that authorizes you to use feedback@example.com, then you would specify the <code>FeedbackForwardingEmailAddressIdentityArn</code> to be arn:aws:ses:us-east-1:123456789012:identity/example.com, and the <code>FeedbackForwardingEmailAddress</code> to be feedback@example.com.</p>
    /// <p>For more information about sending authorization, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/sending-authorization.html">Amazon SES Developer Guide</a>.</p>
    pub feedback_forwarding_email_address_identity_arn:
        ::std::option::Option<::std::string::String>,
    /// <p>An object that contains the body of the message. You can send either a Simple message, Raw message, or a Templated message.</p>
    pub content: ::std::option::Option<crate::types::EmailContent>,
    /// <p>A list of tags, in the form of name/value pairs, to apply to an email that you send using the <code>SendEmail</code> operation. Tags correspond to characteristics of the email that you define, so that you can publish email sending events.</p>
    pub email_tags: ::std::option::Option<::std::vec::Vec<crate::types::MessageTag>>,
    /// <p>The name of the configuration set to use when sending the email.</p>
    pub configuration_set_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the multi-region endpoint (global-endpoint).</p>
    pub endpoint_id: ::std::option::Option<::std::string::String>,
    /// <p>An object used to specify a list or topic to which an email belongs, which will be used when a contact chooses to unsubscribe.</p>
    pub list_management_options: ::std::option::Option<crate::types::ListManagementOptions>,
}
