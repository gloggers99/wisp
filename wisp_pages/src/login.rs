use maud::{html, Markup};

use crate::page_header;

pub fn login_page(flash: Option<&str>) -> Markup {

    html!(
        (page_header())
        
        body class="flex h-screen" {
            // Left panel
            div class="bg-[#282828] text-[#fbf1c7] border border-none border-black w-full md:w-2/5 p-8 md:border-solid flex flex-col justify-center" {
                div class="space-y-8" {
                    h1 class="text-3xl font-bold text-center text-[#fb4934]" { "beigebox" }
                    
                    form class="space-y-6" method="post" action="/login" {
                        div class="flex flex-col space-y-2" {
                            label class="font-semibold text-sm" for="username" { "Username" }
                            input
                                class="px-4 py-2 rounded-md bg-[#3c3836] border border-[#a89984] text-[#fbf1c7] focus:outline-none focus:border-[#fb4934] focus:ring-1 focus:ring-[#fb4934] placeholder-[#bdae93]"
                                type="text"
                                name="username"
                                id="username"
                                placeholder="Username"
                                required;
                        }

                        div class="flex flex-col space-y-2" {
                            label class="font-semibold text-sm" for="password" { "Password" }
                            input
                                class="px-4 py-2 rounded-md bg-[#3c3836] border border-[#a89984] text-[#fbf1c7] focus:outline-none focus:border-[#fb4934] focus:ring-1 focus:ring-[#fb4934] placeholder-[#bdae93]"
                                type="password"
                                name="password"
                                id="password"
                                placeholder="••••••••"
                                required;
                        }

                        button
                            class="w-full bg-[#fb4934] hover:bg-[#cc241d] text-[#fbf1c7] font-semibold py-2 rounded-md transition duration-200 ease-in-out"
                            type="submit" {
                            "Log In"
                        }
                    }
                }
                
                @if let Some(flash) = flash {
                    p class="text-red-500 font-bold mt-4 text-center" { (flash) }
                }
            }

            // Right panel (hidden on mobile)
            div class="bg-[url('https://gruvbox-wallpapers.pages.dev/wallpapers/photography/beach.jpg')] bg-cover bg-center flex-1 hidden md:block" {}

            // Footer badge
            div class="fixed bottom-0 bg-[#a89984] text-[#282828] pl-3 pr-3 mb-3 border-[#fb4934] border-l-4 rounded-tr-md rounded-br-md font-mono" {
                small { "made by Lucas Marta" }
            }
        }
    )
}