use maud::{html, PreEscaped};

pub mod login;

/// Quick component to import the tailwind css sheet and doctype the top of the document. 
/// Theoretically this should be used across all wisp pages, however, I'm sure there will be a case
/// where this won't apply.
pub(crate) fn page_header() -> PreEscaped<String> {
    html!(
        (maud::DOCTYPE)
        
        head {
            meta charset="UTF-8";
            link rel="stylesheet" href="/static/main.css" {}
        }
    )
}

/// "made by Lucas Marta" plug.
pub(crate) fn page_watermark() -> PreEscaped<String> {
    html!(
        div class="fixed bottom-0 bg-[#232946] text-[#f8fafc] pl-3 pr-3 mb-3 border-[#5ecfff] border-l-4 rounded-tr-md rounded-br-md font-mono" {
            small { "made by Lucas Marta" }
        }
    )
}