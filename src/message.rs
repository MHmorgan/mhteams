
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Message<'a> {
    /// Required to always be set to "MessageCard"
    #[serde(rename = "@type")]
    typ: &'a str,

    /// Required to always be set to "https://schema.org/extensions"
    #[serde(rename = "@context")]
    context: &'a str,

    /// Required if the card does not contain a text property, otherwise optional. 
    /// The summary property is typically displayed in the list view in Outlook, 
    /// as a way to quickly determine what the card is all about.
    ///
    /// *Do* always include a summary.  
    /// *Don't* include details in the summary. For example, for a Twitter post, 
    /// a summary might simply read "New tweet from @someuser" without mentioning
    /// the content of the tweet itself.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub summary: String,

    /// Specifies a custom brand color for the card. The color will be displayed
    /// in a non-obtrusive manner.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub theme_color: String,
}

impl<'a> Message<'_> {
    pub fn new() -> Message<'a> {
        Message {
            typ: "MessageCard",
            context: "https://schema.org/extensions",
            ..Default::default()
        }
    }
}