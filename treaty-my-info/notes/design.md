# Overview

General plan:
    - add messages to `treaty-messages` to be seralized as JSON
    - use `treaty-proxy` as the backing instance, will host an http endpoint
        - need to design the API for this
    - use this as the front end to `treaty-proxy` to create and register accounts



# Brainstorm
- Register Account, Delete Account, etc: regular admin functions for a manged `treaty` instance
- When interacting with your `treaty` instance, the `treaty-proxy` will expose an HTTP endpoint will have a wrapper message `treatyRequest` that will take a seralized `treaty-messsage` (CreateUserDatabase, GetHostInfo, etc) as JSON and returns `treatyReply`, also a wrapper over the treaty response
    - this message is recieved at `treaty-proxy`'s HTTP endpoint, which will unwrap the underlying message and leverage the `treaty-client` lib to call `treaty-proxy`'s self-hosted (on localhost) GRPC proxy endpoint
    - all messages are returned back to `my-info` via the response recieved by the `treaty-client`, seralized and wrapped in the `treatyReply` message to be rendered on the webpage


Leverages:
- existing work done in `treaty-admin` 
    - where possible, either share the code OR if possible shell out _the same webpage_ to eliminate duplicate code in `my-info` and `treaty-admin`