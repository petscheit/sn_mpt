# Merkle Trie Cache API

## Overview

This Rust project provides a RESTful API for managing and interacting with batches that are added to a Merkle trie. It includes functionality to create, update, and fetch batches, as well as generate and manage batch proofs. The system utilizes a SQLite database to manage state and supports concurrency control for batch operations.

## Features

- Batch Management: Create, update, and list batches.
- Merkle Proofs: Generate Merkle proofs for each batch operation.
- Concurrency Control: Ensures data consistency through mutexes for batch creation and updates.
- REST API: Easy to use RESTful endpoints for managing batches.

## API Endpoints

This project implements the following API endpoints:

- `GET /batches`: List all batches.
- `GET /batches/{id}`: Fetch a specific batch by ID.
- `POST /batches`: Create a new batch with provided items.
- `PUT /batches/{id}/status/{status}`: Update the status of a batch.


## Getting Started

### Installation

1. Clone the repository:

```bash
git clone <https://your-repository-url.git>
cd your-repository-directory
```

2. Build the project:

```bash
cargo build --release
```

3. Run the server:

```bash
cargo run
```

## Usage
To interact with the API, you can use any HTTP client such as curl or Postman. Below are examples of how to call the API:

### List Batches:

```bash
curl http://localhost:3030/batches
```

### Fetch a Batch:

```bash
curl <http://localhost:3030/batches/{id}>
```

### Create a Batch:

```bash
curl -X POST -H "Content-Type: application/json" -d '[ "ababfefe", "efef0202" ]' http://localhost:3030/batches
```

### Update Batch Status:

```bash
curl -X PUT http://localhost:3000/batches/{id}/status/{new_status}
```

Development
This project is developed using Rust with the Warp web framework for handling HTTP requests. The project is structured to support easy additions of new routes and modifications of existing functionalities.

Testing
Run the following command to execute all tests:

```bash
cargo test
```
