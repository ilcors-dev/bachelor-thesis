# WASI Chat

The repository contains a proof of concept to demonstrate how Wasm can be used to build a distributed application
following a microservice REST API architecture.

It is composed of 4 main services, each one running in a separate Wasm module:
- chat microservice: which handles the CRUD operations on the chats
- message microservice: which handles the CRUD operations on the messages
- session microservice: which handles the user's session which are used to link each other entity of the application. 
It is the only way of 'authenticating' the user since the application voluntarily does not use any authentication mechanism
keep it simple.
- user microservice: a statistics microservice which is used mainly to store / retrieve the currently connected users.
- frontend microservice: a simple frontend which is used to interact with the application which gets served by the
application itself, kinda of a filesystem.

Behind the scenes, the application uses [Spin Framework v0.10.1](https://github.com/fermyon/spin/releases/tag/v0.10.1)
which facilitates the interaction with the Wasm modules, the database and the filesystem.

..todo