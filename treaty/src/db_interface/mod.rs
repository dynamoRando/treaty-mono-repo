// a module containing the trait
pub(crate) mod dbi_actions;
// a module containing the actual database interface application
pub(crate) mod dbi;
// a sqlite native implementation of the database interface trait
#[allow(dead_code, unused_variables)]
pub(crate) mod sqlite;
// a sqlite native implementation of the database interface trait
#[allow(dead_code, unused_variables)]
pub(crate) mod postgres;

/*

A database interface (dbi) is analogous to a Repo in a business domain application.

Treaty abstracts the underlying database type (see the enum `DatabaseType`). This module
contains the trait `dbi_actions` which explains what Treaty expects the underlying database
to return.

The module `dbi` is the actual database interface that returns information to the transport
layer (gRPC, Http, etc).

This module also contains each (hopeful) native implementation of the database interface: Sqlite, Postgres, and so on.

 */
