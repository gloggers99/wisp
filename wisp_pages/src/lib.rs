use maud::{html, PreEscaped};

pub mod login;

pub(crate) fn page_header() -> PreEscaped<String> {
    html!(
        (maud::DOCTYPE)
        
        head {
            meta charset="UTF-8";
            link rel="stylesheet" href="/static/main.css" {}
        }
    )
}
