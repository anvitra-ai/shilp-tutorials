import { ShilpClient, StorageBackendType, IngestSourceType, AttrType, SortOrder, FilterOp } from "@anvitra-ai/shilp-sdk-ts";

const COL = "us-stocks";
const SHILP_URL = "http://localhost:3000";

async function main() {
    // Initialize the client
    const client = new ShilpClient(SHILP_URL);

    // Check health
    const health = await client.healthCheck();
    console.log(`Health: ${health.success}`);

    // Check if the collection exists
    const exists = await checkIfCollectionExists(client);

    console.log("Collection exists:", exists);

    // If collection exists, drop it and add the data again
    // Change this flag to avoid re-indexing the data
    await dropAndAddData(client);

    await searchData(client);
}

async function searchData(client: ShilpClient) {
    // Let's search for technology companies
    let res = await client.searchData({
        collection: COL,
        query: "technology companies",
        fields: ["Name", "Market Cap"],
        limit: 10,
    });
    console.log("Search Results:", res.data[0]["Name"], res.data[0]["Market Cap"]);

    // And see who is the most valued based on the market cap
    res = await client.searchData({
        collection: COL,
        query: "technology companies",
        fields: ["Name", "Market Cap"],
        limit: 10,
        sort: {
            sorts: [{ attribute: "Market Cap", order: SortOrder.Descending }],
        },
    });
    console.log("Most valued technology company:", res.data[0]["Name"], res.data[0]["Market Cap"]);

    // And the least valued among the trillion dollar technology companies
    res = await client.searchData({
        collection: COL,
        query: "technology companies",
        fields: ["Name", "Market Cap"],
        limit: 10,
        sort: {
            sorts: [{ attribute: "Market Cap", order: SortOrder.Ascending }],
        },
        filters: {
            and: [
                {
                    attribute: "Market Cap",
                    op: FilterOp.GreaterThanOrEqual,
                    value: 1000000000000,
                },
            ],
        },
    });
    console.log("Least valued trillion dollar tech company:", res.data[0]["Name"], res.data[0]["Market Cap"]);
}

async function checkIfCollectionExists(client: ShilpClient): Promise<boolean> {
    // List collections
    const collections = await client.listCollections();
    console.log("Collections:", collections.data);

    // Check if our collection exists
    const exists = collections.data.some((c) => c.name === COL);
    console.log("Collection exists -", exists);

    return exists;
}

async function dropAndAddData(client: ShilpClient) {
    // Drop collection if it exists
    try {
        await client.dropCollection(COL);
    } catch (error) {
        // Collection might not exist, which is fine
        console.log("Collection does not exist or already dropped");
    }

    // Create a new collection
    await client.addCollection({
        name: COL,
        has_metadata_storage: true,
        storage_type: StorageBackendType.File,
        reference_storage_type: StorageBackendType.File,
    });

    // Upload the data file
    await client.uploadDataFile("data.csv");
    console.log("File uploaded successfully - data.csv");

    // Begin ingestion
    await client.ingestData({
        file_path: "data.csv",
        source_type: IngestSourceType.File,
        collection_name: COL,

        // Ingestion configuration
        keyword_fields: ["Managing Director", "Sector", "Name"],
        metadata_fields: {
            "Market Cap": AttrType.Float64,
        },
        fields: ["Description"],
    });
    console.log("Data ingestion completed successfully");
}

main().catch((error) => {
    console.error("Error:", error);
    process.exit(1);
});
