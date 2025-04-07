# Auth Microservice

A no-nonsense authentication microservice built in Rust. This project is a work in progress and primarily serves as a training ground.

## What it is

- A basic authentication microservice
- Handles user registration, login, and token management
- Built with Rust

## What it isn't

- Production-ready
- A comprehensive identity management solution
- Over-engineered with buzzwords to impress recruiters

## Current Features

- User registration and login
- Token-based authentication
- Token revocation
- Basic user management

## Tech Stack

- Rust
- SQLite for data storage
- [Insert your web framework here - axum/actix/rocket?]

## Project Status

This is a work in progress. The code works (mostly), but there are rough edges. I'm building this to learn, so expect refactors, breaking changes, and the occasional "what was I thinking?" commit.

Some known design issues, that makes the code no so consistant and pretty hard to test:
- Controllers should have been made public over a trait to be mockable
- Models are currently passive rather than having private (module-relative) functions

## TODO

- [ ] Actually add some unit test
