use crate::{input_box, page_header, page_watermark, wisp_title, InputBoxSpec};

use maud::{html, Markup};

use wisp_database::user::User;

pub fn home_page(user: User) -> Markup {
    html!(
        (page_header())
        
        body class="bg-oxocarbon-background text-oxocarbon-foreground font-lato" {
            div class="flex flex-col h-screen" {
                // Top bar area.
                div class="flex gap-10 border-oxocarbon-1 border-b-2 px-10" {
                    (wisp_title(Some("Home")))
                    
                    form class="flex gap-2" action="/search" method="get" {
                        (input_box(InputBoxSpec {
                            id_name: "query",
                            full_name: "Search",
                            label: false,
                            ..Default::default()
                        }))
                        
                        button
                            class="w-10 bg-[#f9bc60] hover:bg-[#f4845f] text-[#232946] font-semibold my-4 \
                                   rounded-md transition duration-200 ease-in-out"
                            type="submit" {
                            "🔎"
                        }
                    }
                }
                
                // Main content.
                div class="flex-1 px-10 flex" {
                    div class="my-10 w-1/4 h-100 border-oxocarbon-1 border-solid border-2 p-5" {
                        h1 class="text-2xl font-bold" {"Create post"}
                    }
                    div class="flex-1" {
                        
                    }
                }
            }
            
            (page_watermark())
        }
    )
}