## Order Book API in Rust

This project implements a simple order book API in Rust using the Actix web framework.

**Features:**

* Stores buy and sell orders for a single asset pair (defined by `BASE_ASSET` and `QUOTE_ASSET`).
* Processes new orders using the `fill_order` function.
* Provides endpoints for:
    * Placing new orders (`/api/v1/order`)
    * A basic hello world message (`/`)
    * Echoing a request body (`/echo`)

**Dependencies:**

* Rust
* Actix web
* serde
* serde_json

**Running the application:**

1. Ensure you have Rust and Cargo installed.
2. Build the project: `cargo build`
3. Run the application: `cargo run`

This will start the server on port 8080.

**Placing an order:**

Use a tool like `curl` to send a POST request to `/api/v1/order` with a JSON body containing the order details:

```json
{
  "base_asset": "BTC",
  "quote_asset": "USD",
  "price": 10000,
  "quantity": 10,
  "side": "BUY"
}
