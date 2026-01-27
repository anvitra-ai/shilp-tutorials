from shilp import (
    Client,
    AddCollectionRequest,
    IngestRequest,
    SearchRequest,
    CompoundSort,
    SortExpression,
    SortOrder,
    CompoundFilter,
    FilterExpression,
    FilterOp,
    StorageBackendType,
    IngestSourceType,
    AttrType,
)
import time

COL = "us-stocks"
SHILP_URL = "http://localhost:3000"


def main():
    # Initialize the client
    # You can provide custom timeout or use default
    client = Client(SHILP_URL, timeout=600)

    # Check health
    health = client.health_check()
    if not health.success:
        print("Health check failed")
        return
    print(f"Health: {health.success}")

    # Check if the collection exists
    exists = check_if_collection_exists(client)

    print(f"Collection '{COL}' exists: {exists}")

    # If collection exists, drop it and add the data again
    # Change this flag to avoid re-indexing the data
    drop_and_add_data(client)

    search_data(client)


def search_data(client):
    # Let us search for technology companies
    res = client.search_data(
        SearchRequest(
            collection=COL,
            query="technology companies",
            fields=["Name", "Market Cap"],
            limit=10,
        )
    )
    print(f"Search Results: {res.data[0]['Name']}, {res.data[0]['Market Cap']}")

    # And see who is the most valued based on the market cap
    res = client.search_data(
        SearchRequest(
            collection=COL,
            query="technology companies",
            fields=["Name", "Market Cap"],
            limit=10,
            sort=CompoundSort(
                sorts=[
                    SortExpression(attribute="Market Cap", order=SortOrder.DESCENDING)
                ]
            ),
        )
    )
    print(
        f"Most valued tech company: {res.data[0]['Name']}, {res.data[0]['Market Cap']}"
    )

    # And the least valued among the trillion dollar technology companies
    res = client.search_data(
        SearchRequest(
            collection=COL,
            query="technology companies",
            fields=["Name", "Market Cap"],
            limit=10,
            sort=CompoundSort(
                sorts=[
                    SortExpression(attribute="Market Cap", order=SortOrder.ASCENDING)
                ]
            ),
            filters=CompoundFilter(
                and_filters=[
                    FilterExpression(
                        attribute="Market Cap",
                        op=FilterOp.GREATER_THAN_OR_EQUAL,
                        value=1000000000000,
                    )
                ]
            ),
        )
    )
    print(
        f"Least valued trillion dollar tech company: {res.data[0]['Name']}, {res.data[0]['Market Cap']}"
    )


def check_if_collection_exists(client):
    # List collections
    collections = client.list_collections()
    print(f"Collections: {[c.name for c in collections.data]}")

    # Check if collection exists
    exists = any(c.name == COL for c in collections.data)
    print(f"Collection exists: {exists}")

    return exists


def drop_and_add_data(client):
    # Drop collection if exists
    try:
        client.drop_collection(COL)
    except:
        pass

    # Create a new collection
    client.add_collection(
        AddCollectionRequest(
            name=COL,
            has_metadata_storage=True,
            storage_type=StorageBackendType.FILE,
            reference_storage_type=StorageBackendType.FILE,
        )
    )
    print(f"Collection '{COL}' created successfully")

    # Upload the data file
    client.upload_data_file("data.csv")
    print("File uploaded successfully: data.csv")

    # Begin ingestion
    client.ingest_data(
        IngestRequest(
            file_path="data.csv",
            source_type=IngestSourceType.FILE,
            collection_name=COL,
            # Ingestion configuration
            keyword_fields=["Managing Director", "Sector", "Name"],
            metadata_fields={"Market Cap": AttrType.FLOAT64},
            fields=["Description"],
        )
    )
    print("Data ingestion completed successfully")


if __name__ == "__main__":
    main()
