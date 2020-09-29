# Link Shortener
A custom URL shortener implementation for my own website that is written in Rust.
The site is served using the [warp](https://github.com/seanmonstar/warp) framework, with the links stored in a PostgreSQL database and accessed using the [Diesel ORM](https://github.com/diesel-rs/diesel).
There is also a simple administrative UI written in HTML, CSS, and JavaScript for creating, updating, and deleting short links that uses the server's API.
To style the management interface, I used the [Halfmoon UI](https://github.com/halfmoonui/halfmoon) framework.
