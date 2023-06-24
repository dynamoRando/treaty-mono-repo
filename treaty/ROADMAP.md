
# Roadmap

There are few things that I want to implement into `treaty` in the future, time permitting.

# Future

## Native SQL based protocols
Having an intermediate API between your application and a database means that you lose the native features of the underlying database system. I hope in the future to natively implement database protocols such as Postgres and MySQL so that native features of the underlying DBMS is not lost.

## Alternative data storage
Treaty was originally designed against SQL focused databases. There may be in the future a chance to also implement Treaty in a No-SQL like fashion, starting with a key-value orientation. 

## Real-time updates
Another goal is to allow streaming update notifications of changes in partial databases. This use-case is for enterprise level consumers of partial databases wanting real-time notifications of their data in a SaaS application. The goal is outside the scope of version 0.1 of Treaty but the inital thought is to build a pub/sub endpoint to allow integration with stream APIs such as Kafka, Flink, and so on.

# Intermediate term goals

I am debating on if `treaty` should contain more ORM-like behaviors. For example, it would be nice if an application developer could simply add macros to their structs with the intended storage location, and `treaty` handles the rest. As an example (this does not exist yet):

```Rust
use treaty_policy;

#[Host]
struct Inventory {
    id: u32,
    desc: String
}

#[Participant]
struct Address {
    number: String,
    direction: String,
    city: String
    state_province: String,
    postal_code: String
}
```

And Treaty would just simply know what to do with the structs (where to be saved at) by calling something like `inventory.save()`. I would imagine this would be behind a feature flag. This is related to the "Alternative data storage" section earlier in the roadmap.

# Near term goals
## For v0.1 -
- Remove and handle gracefully locations where `.unwrap()` has been used.
- Handle "non-happy" path situations around timeouts and database errors.
- Improve documentation.
- Offer ability to download Sqlite databases in `treaty-admin` and `treaty-my-info`. 
    - Right now both admin interfaces allow CRUD operations, but no easy way for a participant to download their data.
- Ensure there is a migration path between data originally hosted at a `treaty-proxy` instance to a stand-alone `treaty` instance and vice versa.
- Add saving the treaty version into each database. This would be in support of providing schema migrations between new versions of Treaty. 
- Break out services into authenticated and public services.

## For v0.2 -
- Offer ability specify cert for HTTPS in Tonic
- Support gRPC-Web if possible.

## Pre 1.0 - 
- Offer SSR for `treaty-admin` and `treaty-my-info`
    - Right now both websites are written in Yew, which doesn't have standard SSR (versus CSR).
- Offer a TUI for admin of a treaty instance. 
    - This would be a REPL interface, similiar to `psql` or `ksqldb` 
- Offer support for MySQL and Postgres.

## Post 1.0 -
- Potentially offer a NoSQL alternative storage, likely in a Key-Value fashion. Potentially define data contracts in a markup file, either YAML or TOML.