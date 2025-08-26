use maud::{html, Markup, PreEscaped};

use crate::{page_header, page_watermark};

/// Contains the login page (left (login) and right (aesthetic) panels).
fn login_container(flash: Option<&str>) -> PreEscaped<String> {
    html!(
        body class="flex h-screen" {
            // Left panel, where the login action really happens.
            (login_form(flash))

            // Aesthetic right panel (hidden on mobile). Practically useless but looks cool and 
            // modern.
            // TODO: Find a true fitting image for wisp, not just something random I found online.
            div class="bg-[url('https://github.com/linuxdotexe/nordic-wallpapers/raw/master/wallpapers/ign-supertree-grove-singapore.png')] bg-cover bg-center flex-1 hidden md:block" {}
            
            (page_watermark())
        }
    )
}

/// Login input section.
fn login_form(flash: Option<&str>) -> PreEscaped<String> {
    html!(
        div class="bg-[#11151c] text-[#f8fafc] border border-none border-[#0d1117] w-full md:w-2/5 lg:w-1/4 p-8 md:border-solid flex flex-col justify-center" {
            div class="space-y-8" {
                h1 class="text-3xl font-bold text-center text-[#5ecfff]" { "wisp" }
                
                form class="space-y-6" method="post" action="/login" {
                    div class="flex flex-col space-y-2" {
                        label class="font-semibold text-sm" for="username" { "Username" }
                        input
                            class="animated-input px-4 py-10 rounded-md bg-[#232946] border border-[#0d1117] text-[#f8fafc] focus:outline-none focus:border-[#f9bc60] focus:ring-1 focus:ring-[#f9bc60] placeholder-[#b8c1ec]"
                            type="text"
                            name="username"
                            id="username"
                            placeholder="Username"
                            required;
                    }

                    div class="flex flex-col space-y-2" {
                        label class="font-semibold text-sm" for="password" { "Password" }
                        input
                            class="animated-input px-4 pt-10 pb-10 rounded-md bg-[#232946] border border-[#0d1117] text-[#f8fafc] focus:outline-none focus:border-[#f9bc60] focus:ring-1 focus:ring-[#f9bc60] placeholder-[#b8c1ec]"
                            type="password"
                            name="password"
                            id="password"
                            placeholder="••••••••"
                            required;
                    }

                    button
                        class="w-full bg-[#f9bc60] hover:bg-[#f4845f] text-[#232946] font-semibold py-2 rounded-md transition duration-200 ease-in-out"
                        type="submit" {
                        "Log In"
                    }
                }
            }
            
            @if let Some(flash) = flash {
                p class="text-[#ff6f91] font-bold mt-4 text-center" { (flash) }
            }
        }
    )
}

pub fn login_page(flash: Option<&str>) -> Markup {
    html!(
        (page_header())

        // Add animation styles for input boxes
        style {
            "
            .animated-input {
                transition: border-color 0.25s cubic-bezier(0.4,0,0.2,1), box-shadow 0.25s cubic-bezier(0.4,0,0.2,1);
            }
            .animated-input:focus {
                border-color: #f9bc60 !important;
                box-shadow: 0 0 0 3px rgba(249,188,96,0.25);
            }
            "
        }
        
        (login_container(flash))
    )
}