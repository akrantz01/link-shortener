# Link Shortener
A custom URL shortener implementation for my own website that is written in Rust.
The site is served using the [warp](https://github.com/seanmonstar/warp) framework, with the links stored in a PostgreSQL database and accessed using the [Diesel ORM](https://github.com/diesel-rs/diesel).
There is also a simple administrative UI written in HTML, CSS, and JavaScript for creating, updating, and deleting short links that uses the server's API.
To style the management interface, I used the [Halfmoon UI](https://github.com/halfmoonui/halfmoon) framework.

### Special Link Names
When navigating to the service without specifying a path, you will be shown a 404 page by default.
You can create a link with the name `root` that will be redirected to when you go to the service without specifying the path.
Note that the name `root` will also be accessible from `/root`. 

## Deployment
This can either be run just as a standalone binary or in a Docker container.
However, you must have a supporting PostgreSQL database running to store the link mappings.
Both are configured using environment variables as specified in [`.env.sample`](/.env.sample).
**NOTE:** When running in a docker container, you must have the address listening on `0.0.0.0` or `::1` so it can be accessible from outside the container.

### Standalone Binary
If you would like a standalone binary, one can be retrieved from the [Releases](https://github.com/akrantz01/link-shortener/releases/latest) tab.
Currently, only Linux binaries are provided.
However, if your OS is not supported, follow the below instructions:
1. Clone the repository: `git clone git@github.com:akrantz01/link-shortener.git`
1. Build the binary: `cargo build --release`
1. The resulting binary can be found at `./target/releases/link-shortener`

### Docker
A Docker image is available on [Docker Hub](https://hub.docker.com/r/akrantz01/link-shortener) with the latest version.
Simply run it with `docker run -d --name link-shortener -e DATABASE_URL=<postgres> akrantz01/link-shortener:latest`, where `<postgres>` is the connection URL for the database.
Alternatively, you can use the provided [`docker-compose.yml`](docker-compose.yml) so it is just plug-and-play.
