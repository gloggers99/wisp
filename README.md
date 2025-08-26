# `wisp`
An evolution of social media.

# Structure

The `wisp` project is seperated between several crates.

- `wisp`

This is where everything comes together into a binary. The web server is powered by 
[rocket](https://crates.io/crates/rocket).

- `wisp_core`

> [!CAUTION] 
> This crate is planned on being removed.

The core is where static variables and other code is stored for usage around the overall project.

- `wisp_database`

The database is where user data is stored. Security and speed are top priority here and neither are sacrificed. With the
backend being [sled](https://crates.io/crates/sled) with [bincode](https://crates.io/crates/bincode) encoding.

As of right now there is no encryption, however, there is no possible way of someone gaining access of the database. In
the future [argon2](https://crates.io/crates/argon2) will be deployed.

- `wisp_pages`

This is where the frontend lies. Written using [maud](https://crates.io/crates/maud) and 
[tailwind](https://tailwindcss.com/) (believe it or not!) 

- `wisp_session_manager`

This is where user authentication happens. This crate functions with `wisp_database` to create and monitor sessions.