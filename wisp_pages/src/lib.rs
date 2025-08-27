use maud::{html, Markup};

pub mod login;
pub mod signup;

pub(crate) static COMPILED_CSS: &'static str = include_str!("../tailwind/target/main.css");

/// Quick component to import the tailwind css sheet and doctype the top of the document. 
/// Theoretically this should be used across all wisp pages, however, I'm sure there will be a case
/// where this won't apply.
pub(crate) fn page_header() -> Markup {
    html!(
        (maud::DOCTYPE)
        
        head {
            meta charset="UTF-8";
            style {
                (COMPILED_CSS)
            }
        }
    )
}

/// "made by Lucas Marta" plug.
pub(crate) fn page_watermark() -> Markup {
    html!(
        div class="fixed bottom-0 bg-oxocarbon-1 text-oxocarbon-foreground pl-3 pr-3 mb-3 \
                   border-oxocarbon-magenta border-l-4 rounded-tr-sm rounded-br-sm font-mono" {
            small { "made by Lucas Marta" }
        }
    )
}

pub(crate) fn input_box(id_name: &str,
                        full_name: &str,
                        password: bool) -> Markup {
    html!(
        div class="flex flex-col my-4" {
            label class="font-semibold text-sm mb-2" for=(id_name) { (full_name) }
            input
            class="px-4 py-3 rounded-md bg-oxocarbon-1 border border-[#0d1117] \
                               text-oxocarbon-foreground focus:outline-none \
                               focus:border-oxocarbon-magenta focus:ring-1 focus:ring-[#f9bc60] \
                               placeholder-oxocarbon-2"
            type=(if password { "password" } else { "text" })
            name=(id_name)
            id=(id_name)
            placeholder=(if password { "••••••••" } else { full_name })
            required;
        }
    )
}

