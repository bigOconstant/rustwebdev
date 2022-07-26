# Rust web Development

This repository is meant to show case how to create a traditional web application in rust.

It has a docker-compose development environment. Uses diesel as the orm and it is preinstalled.

## developement

A docker-compose vscode environment is provide to make development easier. Start with 

`docker-compose -f Docker/Development/docker-compose.yml up `

exec into the main container. 

Run the diesel migrations with `diesel migration run`

run the application with `cargo run`.