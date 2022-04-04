# Rocket Mongo
[![build](https://github.com/Speccy-Rom/rust_rocket_mongo/actions/workflows/ci.yml/badge.svg)](https://github.com/Speccy-Rom/rust_rocket_mongo/actions/workflows/ci.yml)
![](https://img.shields.io/badge/os-windows%7Clinux%7Cmacos-orange)
![](https://img.shields.io/badge/platform-intel%7Carm-yellowgreen)
![GitHub](https://img.shields.io/github/license/Speccy-Rom/rust_rocket_mongo?style=plastic)

A Simple Skeleton API Rest server using [Rocket](https://rocket.rs/) with the backend database as [MongoDB](https://www.mongodb.com/).

### Features
- Custom config file defining:
    - server host ip and port to listen
    - enable/disable ssl with ssl cert auto generation
    - mongodb configurations
- Use the `x-api-key` header to validate `API Keys`
- `Restrict` a client connecting IP Addresses to the endpoints using `Allow ACL`
- `Restrict` endpoints using the `Allow ACL`

### Requirements

- Rust 1.56+ (2021 edition)

### Compile