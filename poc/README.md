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

It uses Docker and Docker Compose to handle the persistence layer.

The frontend is a simple Typescript+React application bundled with vite.
See the [package.json](./web/package.json) file for the list of dependencies.
## How to run the application

### Prerequisites

To run the application, you need to have the following tools installed:

[Backend]
- [Rust](https://www.rust-lang.org/tools/install)
- [Wasmtime](https://wasmtime.dev/)
- [Spin Framework v0.10.1](https://github.com/fermyon/spin/releases/tag/v0.10.1)
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)\

[Frontend]
- [Node.js](https://nodejs.org/en/download/)

### Run the application
Steps:

1. Go to the `poc` directory
    ```bash
    cd ./poc
    ```
2. Run the docker containers
    ```bash
    docker-compose up -d
    ```
3. Build the Wasm modules & start the backend
    ```bash
    spin build --up
    ```
4. Start the frontend
    ```bash
    cd ./web
    npm install
    npm run start
    ```
5. Open the application in your browser at http://localhost:3000

You can now run the application & play with it.