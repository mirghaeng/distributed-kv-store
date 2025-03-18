# Key-value store

Inspired by the [research paper](https://www.allthingsdistributed.com/files/amazon-dynamo-sosp2007.pdf) for Amazon Dynamo/DynamoDB.

A replicated fault-tolerant key-value store with causal consistency, supporting key-value store operations:

- GET request (for retrieving the value of key `<key>`)
- PUT request (for adding a new key or updating value of an existing key `<key>`)
- DELETE request (for deleting key `<key>`)

to the `/key-value-store/<key>` endpoint at a replica.

The store returns a response in JSON format with the appropriate HTTP status code.

## Set up Environment
Install [rustup](https://rustup.rs/), then compile and run the program
```sh
cargo run
```