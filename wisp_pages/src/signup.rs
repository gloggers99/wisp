use maud::{html, Markup};
use crate::{flash_message, input_box, page_header, page_watermark, wisp_title, InputBoxSpec};
use crate::login::canvas;

pub fn form() -> Markup {
    html!(
        form class="my-6 font-lato" method="post" action="signup" {
            (input_box(InputBoxSpec {
                id_name: "email", 
                full_name: "Email", 
                ..Default::default()
            }))

            (input_box(InputBoxSpec {
                id_name: "username", 
                full_name: "Username",
                ..Default::default()
            }))
            (input_box(InputBoxSpec {
                id_name: "password", 
                full_name: "Password", 
                password: true,
                ..Default::default()
            }))
            
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
            (wisp_title(Some("Sign Up")))
            
            div class="h-1/2" {
                (form())
            
                (flash_message(flash))
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
            (page_watermark())
        }
    )
}