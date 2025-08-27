use maud::{html, Markup};

pub mod login;
pub mod signup;
pub mod home;

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

pub(crate) struct InputBoxSpec<'a> {
    pub id_name: &'a str,
    pub full_name: &'a str,
    
    pub label: bool,
    pub password: bool,
    
    pub additional_css: &'a str
} impl Default for InputBoxSpec<'_> {
    fn default() -> Self {
        Self {
            id_name: "input",
            full_name: "Input",
            label: true,
            password: false,
            additional_css: ""
        }
    }
}

pub(crate) fn input_box(spec: InputBoxSpec) -> Markup {
    html!(
        div class="flex flex-col my-4" {
            @if spec.label {
                label class="font-semibold text-sm mb-2" for=(spec.id_name) { (spec.full_name) }
            }
            input
            class=(format!("px-4 py-1 rounded-md bg-oxocarbon-1 border border-[#0d1117] \
                               text-oxocarbon-foreground focus:outline-none \
                               focus:border-oxocarbon-magenta focus:ring-1 focus:ring-[#f9bc60] \
                               placeholder-oxocarbon-2 {}", spec.additional_css))
            type=(if spec.password { "password" } else { "text" })
            name=(spec.id_name)
            id=(spec.id_name)
            placeholder=(if spec.password { "••••••••" } else { spec.full_name })
            required;
        }
    )
}

pub(crate) fn flash_message(message: Option<&str>) -> Markup {
    html!(
        @if let Some(message) = message {
            h1 class="text-oxocarbon-magenta text-center font-lato text-lg" { (message) }
        } 
    )
}

pub(crate) fn wisp_title(subtext: Option<&str>) -> Markup {
    html!(
        div {
            h1 class="text-oxocarbon-magenta inline-block text-5xl font-lato font-bold" {
                "wisp"
            }

            @if let Some(subtext) = subtext {
                h1 class="ml-1 inline-block text-2xl font-lato font-bold" { (subtext) }
            }
        }
    )
}