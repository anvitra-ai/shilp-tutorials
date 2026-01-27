# Simple Ingestion and Search Tutorial

This tutorial demonstrates how to use the Shilp Python SDK to:

- Create a collection
- Upload and ingest data from a CSV file
- Perform various search operations with filtering and sorting

## Prerequisites

- Python 3.7 or higher
- Shilp server running at `http://localhost:3000`

## Installation

1. Create and activate a virtual environment:

```bash
# Create virtual environment
python -m venv venv

# Activate on macOS/Linux
source venv/bin/activate

# Activate on Windows
# venv\Scripts\activate
```

2. Install the required dependencies:

```bash
pip install -r requirements.txt
```

Or install the Shilp SDK directly:

```bash
pip install shilp-sdk
```

## Running the Tutorial

Make sure your virtual environment is activated, then run:

```bash
python main.py
```

To deactivate the virtual environment when you're done:

```bash
deactivate
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

- **Client Initialization**: Creating a Shilp client with custom timeout
- **Collection Management**: Creating and checking collections
- **File Upload**: Uploading data files to Shilp storage
- **Data Ingestion**: Configuring keyword fields, metadata fields, and searchable fields
- **Search**: Performing semantic search with various options
- **Filtering**: Using compound filters with comparison operators
- **Sorting**: Ordering results by metadata fields

## Learn More

- [Shilp Python SDK Documentation](https://pypi.org/project/shilp-sdk/)
- [Shilp SDK GitHub](https://github.com/anvitra-ai/shilp-sdk-py)
