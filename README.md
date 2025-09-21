# GISTORY

A tool to create custom commit patterns to display on your GitHub profile

It includes a CLI, a Rust library, a REST API and a webpage

## How it works

- Github updates your commit graph retroactively using the timestamp of the commit itself, not when the commit
  pushed to Github
- Gistory creates a bunch of empty commits with desired timestamp to simulate the commit history of the repo

## Project structure

- `gistory` - Rust CLI & library, contain the core logic
- `webserver` - Axum webserver, serving the REST API
- `frontend` - React webpage to interact with the API

## Development

