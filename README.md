# `wisp`
A social media experiment

# Structure

The `wisp` project is seperated between several crates.

- `wisp`

This is where everything comes together into a binary. The web server is powered by 
[rocket](https://crates.io/crates/rocket).

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

# Security

As of right now there should be ZERO vulnerabilities in `wisp`.

## Authentication process

When a user logs in a unique session UUID is given to the user in a private cookie.

As long as this user has this cookie in their browser for the session time (check `SESSION_TIME` in the 
`session_manager.rs`) they will automatically be let into the website as an authenticated user.

When the time runs out the session UUID will permanently be invalid.
