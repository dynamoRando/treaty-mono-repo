// all errors
pub mod error;

// the treaty instance
#[allow(dead_code)]
pub mod service;

// the settings for a treaty instance
#[allow(dead_code)]
pub mod settings;

// handler for the transport implementation of a user service.
// a user service is the API to be consumed by application developers
#[doc(hidden)]
pub mod user_service_handler;

// handler for the transport implementation of a data service.
// a data service is the API to be consumed by _other_ treaty instances.
#[doc(hidden)]
pub mod data_service_handler;

// database interface (dbi):
//  contains the structs and traits that abstract communications to a
//  database (sqlite, mysql, postgres). this is analogous to a repo (repository)
//  in a business line application
#[allow(dead_code)]
pub(crate) mod db_interface;

// all objects that can be returned from a db interface (dbi)
#[allow(dead_code)]
mod models;

// treaty defaults
#[allow(dead_code)]
#[doc(hidden)]
pub mod defaults;

// the protobuf definition of the treaty service
#[allow(dead_code)]
pub mod treaty_proto;

// custom impl for protobuf objects
#[allow(dead_code)]
mod treaty_proto_impl;

// functions related to encryption, primary used by authentication (username, password)
#[allow(dead_code)]
#[doc(hidden)]
pub mod crypt;

// default SQL statements, usually in reference to configuring a treaty instance
#[allow(dead_code)]
pub(crate) mod sql_text;

// struct representing a Json Web Token
#[allow(dead_code)]
#[doc(hidden)]
pub mod jwt;

// abstraction over Antlr, using in parsing SQL statements to determine
// where data should live
#[allow(dead_code)]
pub(crate) mod query_parser;

// a client of the data_service_handler: use to talk to databases that are remote
// from this treaty instance (in other words for talking to partial databases)
#[allow(dead_code)]
pub(crate) mod remote;

// module for non-gRPC implentations: Http (and hopefully Websockets, Postgres, MySQL)
#[allow(dead_code)]
#[doc(hidden)]
pub mod alt_transport;
