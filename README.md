# NRF Cloud Lib

A Rust library for interacting with the NRF Cloud API.

## Features

- Simple and intuitive API
- Async/await support with Tokio
- JSON serialization/deserialization with Serde
- Typed request and response handling
- Customizable base URL

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nrfcloudlib = "0.1.0"
```

## Usage

### Basic GET Request

```rust
use nrfcloudlib::NRFCloudClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "your_api_token_here";
    let client = NRFCloudClient::new(token);
    
    // Simple GET request
    let response = client.get("/devices").await?;
    println!("Response: {}", response);
    
    Ok(())
}
```

### GET Request with Query Parameters

```rust
use nrfcloudlib::NRFCloudClient;
use serde::Serialize;

#[derive(Serialize)]
struct QueryParams {
    limit: u32,
    offset: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "your_api_token_here";
    let client = NRFCloudClient::new(token);
    
    let params = QueryParams {
        limit: 10,
        offset: 0,
    };
    
    let response = client.get_with_params("/devices", &params).await?;
    println!("Response: {}", response);
    
    Ok(())
}
```

### GET Request with JSON Deserialization

```rust
use nrfcloudlib::NRFCloudClient;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Device {
    id: String,
    name: String,
    // Add other fields as needed
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "your_api_token_here";
    let client = NRFCloudClient::new(token);
    
    // GET request with JSON deserialization
    let device: Device = client.get_json("/devices/device-id").await?;
    println!("Device: {:?}", device);
    
    Ok(())
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
