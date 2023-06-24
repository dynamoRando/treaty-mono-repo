/*
This module contains non-gRPC transport implementations for treaty.
The hope is that we will eventually be able to support:

- Http (implemented but not converted from `Treaty` codebase)
- Websockets
- Native Postgres
- Native MySQL

 */

pub mod http;
