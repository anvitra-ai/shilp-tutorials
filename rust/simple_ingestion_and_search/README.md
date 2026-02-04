# Simple Ingestion and Search Tutorial (Rust)

This tutorial demonstrates how to use the Shilp Rust SDK to:

- Create a collection
- Upload and ingest data from a CSV file
- Perform various search operations with filtering and sorting

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Shilp server running at `http://localhost:3000`

## Installation

1. Clone the repository and navigate to the Rust tutorial:

```bash
cd rust/simple_ingestion_and_search
```

2. The dependencies are already configured in `Cargo.toml`:

```toml
[dependencies]
shilp-sdk = "0.12.2"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
serde_json = "1.0"
```

## Running the Tutorial

To run the tutorial, use `cargo run`:

```bash
cargo run
```

## What This Tutorial Does

1. **Health Check**: Verifies the Shilp server is running
2. **Collection Management**: Creates a collection named "us-stocks" (or uses existing one)
3. **Data Upload**: Uploads the `data.csv` file containing US stock market data
4. **Data Ingestion**: Ingests the CSV data into the collection with:
   - Keyword fields: Managing Director, Sector, Name
   - Metadata field: Market Cap (as float64)
   - Searchable fields: Description
5. **Search Operations**:
   - Basic search for "technology companies"
   - Search with sorting (by Market Cap, descending)
   - Search with filtering and sorting (trillion dollar companies, ascending)

## Expected Output

The tutorial will show:

- Health check status
- List of existing collections
- Search results for technology companies
- The most valued tech company by market cap
- The least valued trillion dollar tech company

## Data Format

The `data.csv` file contains information about US companies with the following columns:

- Name
- Description
- Sector
- Products and Services
- Market Cap
- Managing Director

## Key Concepts Demonstrated

- **Client Initialization**: Creating a Shilp client
- **Async/Await**: Using `tokio` for asynchronous operations
- **Collection Management**: Creating and checking collections
- **File Upload**: Uploading data files to Shilp storage
- **Data Ingestion**: Configuring keyword fields, metadata fields, and searchable fields
- **Search**: Performing semantic search with various options
- **Filtering**: Using compound filters with comparison operators
- **Sorting**: Ordering results by metadata fields

## Learn More

- [Shilp Rust SDK on Crates.io](https://crates.io/crates/shilp-sdk)
- [Official Shilp Documentation](https://github.com/anvitra-ai/shilp-sdk-rust)
