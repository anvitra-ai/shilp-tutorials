# Shilp TypeScript Tutorial - Simple Ingestion and Search

This tutorial demonstrates how to use the Shilp SDK for TypeScript to perform simple data ingestion and search operations.

## Prerequisites

- Node.js (v18 or higher recommended)
- npm or yarn
- Shilp server running at `http://localhost:3000`

## Installation

Install the dependencies:

```bash
npm install
```

## Usage

### Development Mode

Run the tutorial directly with ts-node:

```bash
npm run dev
```

### Production Mode

Build and run the compiled JavaScript:

```bash
npm run build
npm start
```

## What This Tutorial Does

1. **Health Check**: Verifies the Shilp server is running
2. **Collection Management**:
   - Lists existing collections
   - Checks if the `us-stocks` collection exists
   - Drops and recreates it if needed
3. **Data Ingestion**:
   - Uploads a CSV file containing US stock data
   - Ingests the data with proper field configurations
   - Sets up keyword fields and metadata
4. **Search Operations**:
   - Basic search for "technology companies"
   - Sorted search (descending) to find the most valued tech company
   - Filtered search to find the least valued trillion-dollar tech company

## Data Structure

The tutorial uses a CSV file (`data.csv`) containing US stock information with the following fields:

- Name
- Description
- Sector
- Products and Services
- Market Cap
- Managing Director

## SDK Features Demonstrated

- Client initialization
- Health check
- Collection listing and management
- File upload
- Data ingestion with metadata
- Search with sorting and filtering

## Configuration

You can modify the following constants in `src/main.ts`:

- `COL`: Collection name (default: "us-stocks")
- `SHILP_URL`: Shilp server URL (default: "http://localhost:3000")

## Learn More

For more information about the Shilp SDK, visit the [official documentation](https://github.com/anvitra-ai/shilp-sdk-ts).
