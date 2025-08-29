use crate::{page_header, page_watermark};

use maud::{html, Markup};
use wisp_database::database::Database;
use wisp_database::user::User;
use wisp_session_manager::admin_user::AdminUser;

fn database_grid(users: Vec<User>) -> Markup {
    html!(
        div class="flex flex-row" {
            div class="w-1/4 bg-oxocarbon-3 border-oxocarbon-1 border-2" {
                "EMAIL"
            }
            div class="w-1/4 bg-oxocarbon-3 border-oxocarbon-1 border-2" {
                "USERNAME"
            }
            div class="w-1/4 bg-oxocarbon-3 border-oxocarbon-1 border-2" {
                "PASSWORD"
            }
            div class="w-1/4 bg-oxocarbon-3 border-oxocarbon-1 border-2" {
                "ACTION"
            }
        }
        @for user in users {
            div class="flex flex-row" {
                div class="w-1/4 border-oxocarbon-1 border-2" {
                    (user.email())
                }
                div class="w-1/4 border-oxocarbon-1 border-2" {
                    (user.username())
                }
                div class="w-1/4 border-oxocarbon-1 border-2" {
                    (user.password())
                }
                div class="w-1/4 border-oxocarbon-1 border-2 flex flex-row" {
                    button class="flex-1 border-oxocarbon-magenta border-2 bg-oxocarbon-3" {
                        "Delete"
                    }
                    button class="flex-1 border-oxocarbon-magenta border-2 bg-oxocarbon-3" {
                        "Asdf"
                    }
                }
            }
        }
    )
}

pub fn admin_page(user: &AdminUser,
                  database: &Database) -> Markup {
    let users = database.get_all_users().unwrap();
    
    html!(
        (page_header())
        
        body class="font-lato text-oxocarbon-foreground bg-oxocarbon-background" {
            div class="flex h-screen" {
                div class="px-5 border-oxocarbon-magenta border-r-2" {
                    h1 class="text-xl font-bold" { "ADMIN PANEL" }
                    p { "Welcome, " (user.authenticated_user().user.username()) "!" }
                }
                
                div class="flex-1 overflow-y-scroll" {
                    (database_grid(users))
                }
            }
            
            (page_watermark())
        }
    )
}