use maud::{html, Markup};
use crate::{input_box, page_header, page_watermark};
use crate::login::canvas;

pub fn form() -> Markup {
    html!(
        form class="my-6 font-lato" method="post" action="signup" {
            (input_box("email", "Email", false))
            (input_box("username", "Username", false))
            (input_box("password", "Password", true))

            button
                class="w-full bg-[#f9bc60] hover:bg-[#f4845f] text-[#232946] font-semibold py-2 \
                       rounded-md transition duration-200 ease-in-out"
                type="submit" {
                "Sign Up"
            }
        }
    )
}

fn left_side(flash: Option<&str>) -> Markup {
    html!(
        div class="w-screen md:w-1/3 lg:w-1/4 bg-oxocarbon-background text-oxocarbon-foreground \
                   flex flex-col justify-center align-center px-20 md:px-10" {
            @if let Some(message) = flash {
                p class="text-oxocarbon-magenta text-center" { (message) }
            }
            
            div {
                h1 class="text-oxocarbon-magenta inline-block text-5xl font-lato font-bold" { 
                    "wisp" 
                }
                
                h1 class="ml-1 inline-block text-2xl font-lato font-bold underline" { "Sign Up" }
            }
            
            div class="h-1/2" {
                (form())
            }
        }
    )
}

pub fn signup_page(flash: Option<&str>) -> Markup {
    html!(
        (page_header())
        
        body class="flex h-screen" {
            (left_side(flash))
            (canvas())
        }
        
        (page_watermark())
    )
}