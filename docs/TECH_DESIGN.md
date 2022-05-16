# Counter

A simple app that demos Fuel functionality featuring

- Web API
- Smart contract
- Frontend

### Frontend
- A simple button that can be pressed
- Pressing `Count` button sends a `POST` request to a web server to increment
    the number of counts
- Pressing the `Get` button sends a `GET` request to the webserver which returns
    how many counts have been made

### Web API
- Using whatever simple/thin Rust HTTP library is available
- Only needs to support `GET /api/count` and `POST /api/count`
- Is the middleman between the client and Fuel-land

### Smart contract
- A contract created with `forc` tooling
- Keeps state of how many `Count`s have been made
- Supports `increment_count()` and `get_count()`
- Performs no authority checks?
