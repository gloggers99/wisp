use maud::{html, Markup};

#[get("/home")]
pub fn home_get() -> Markup {
    html!(
        (maud::DOCTYPE)
        html {
            body {
                p { "Hello!" }
            }
        }
    )
}