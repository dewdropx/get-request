# Rust GET Request

A simple Rust project that demonstrates how to send an HTTP GET request using the `reqwest` crate, read the server's response, and display the HTTP status code, response headers, and response body.

## Features

- Sends an HTTP GET request to [HTTPBin](https://httpbin.org/get).
- Reads the HTTP response body into a `String`.
- Displays the HTTP status code.
- Prints the response headers.
- Displays the response body in JSON format.
- Handles I/O and HTTP errors using `error-chain`.

## Technologies Used

- **Rust** — Programming language.
- **Reqwest** — HTTP client library.
- **Error Chain** — Error handling library.
- **HTTPBin** — Test endpoint for HTTP requests.

## Prerequisites

Install [Rust](https://www.rust-lang.org/tools/install) and Cargo before running the project.

## Installation and Usage

**1. Clone the repository**

```bash
git clone https://github.com/dewdropx/get-request.git
cd get-request
```

**2. Run the project**

```bash
cargo run
```

Cargo downloads the required dependencies, compiles the project, and executes the program.

## Example Output

The exact output may vary depending on the server response.

```text
Status: 200 OK
Headers:
{
    "content-type": "application/json",
    ...
}

Body:
{
  "args": {},
  "headers": {
    ...
  },
  "url": "http://httpbin.org/get"
}
```

## Dependencies

The project uses the following crates in `Cargo.toml`:

- `reqwest` with the `blocking` feature for synchronous HTTP requests - make sure to have the latest version
- `error-chain` for propagating and handling errors.

## Limitations

- Uses a fixed URL rather than accepting a URL from command-line arguments.
- Depends on an active internet connection and the availability of HTTPBin.
