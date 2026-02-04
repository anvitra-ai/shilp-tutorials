use anyhow::{Result, Context};
use shilp_sdk::Client;
use shilp_sdk::models::{
    AddCollectionRequest, IngestRequest, SearchRequest,
    StorageBackendType, IngestSourceType, AttrType, SortOrder, FilterOp,
    CompoundSort, SortExpression, CompoundFilter, FilterExpression,
};
use std::collections::HashMap;
use std::path::Path;

const COL: &str = "us-stocks";
const SHILP_URL: &str = "http://localhost:3000";

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the client
    let client = Client::new(SHILP_URL);

    // Check health
    let health = client.health_check().await.context("Health check failed")?;
    println!("Health: {}", health.success);

    // Check if the collection exists
    let exists = check_if_collection_exists(&client).await?;
    println!("Collection '{}' exists: {}", COL, exists);

    // If collection exists, drop it and add the data again
    // Change this flag to avoid re-indexing the data
    drop_and_add_data(&client).await?;

    search_data(&client).await?;

    Ok(())
}

async fn check_if_collection_exists(client: &Client) -> Result<bool> {
    // List collections
    let collections = client.list_collections().await.context("Failed to list collections")?;
    
    let names: Vec<_> = collections.data.iter().map(|c| &c.name).collect();
    println!("Collections: {:?}", names);

    let exists = collections.data.iter().any(|c| c.name == COL);
    println!("Collection exists: {}", exists);

    Ok(exists)
}

async fn drop_and_add_data(client: &Client) -> Result<()> {
    // Drop collection if exists
    let _ = client.drop_collection(COL).await;

    // Create a new collection
    client.add_collection(&AddCollectionRequest {
        name: COL.to_string(),
        has_metadata_storage: Some(true),
        storage_type: Some(StorageBackendType::File),
        reference_storage_type: Some(StorageBackendType::File),
        no_reference_storage: None,
        enable_pq: None,
    }).await.context("Failed to add collection")?;
    
    println!("Collection '{}' created successfully", COL);

    // Upload the data file
    client.upload_data_file(Path::new("data.csv")).await.context("Failed to upload file")?;
    println!("File uploaded successfully: data.csv");

    // Begin ingestion
    let mut metadata_fields = HashMap::new();
    metadata_fields.insert("Market Cap".to_string(), AttrType::Float64);

    client.ingest_data(&IngestRequest {
        file_path: Some("data.csv".to_string()),
        source_type: Some(IngestSourceType::File),
        collection_name: COL.to_string(),
        keyword_fields: Some(vec![
            "Managing Director".to_string(),
            "Sector".to_string(),
            "Name".to_string(),
        ]),
        metadata_fields: Some(metadata_fields),
        fields: vec!["Description".to_string()],
        database_name: None,
        mongo_collection: None,
        query: None,
        mongo_fetch_batch_size: None,
        id_field: None,
        expiry_field: None,
        embedding_provider: None,
        embedding_model: None,
        ingestion_batch_size: None,
    }).await.context("Data ingestion failed")?;

    println!("Data ingestion completed successfully");

    Ok(())
}

async fn search_data(client: &Client) -> Result<()> {
    // Basic search for technology companies
    let res = client.search_data(&SearchRequest {
        collection: COL.to_string(),
        query: Some("technology companies".to_string()),
        fields: Some(vec!["Name".to_string(), "Market Cap".to_string()]),
        limit: Some(10),
        weights: None,
        max_distance: None,
        filters: None,
        sort: None,
        vector_query: None,
    }).await.context("Search failed")?;

    if let Some(first) = res.data.first() {
        println!("Search Results: {}, {:?}", first["Name"], first["Market Cap"]);
    }

    // Search with sorting (by Market Cap, descending)
    let res = client.search_data(&SearchRequest {
        collection: COL.to_string(),
        query: Some("technology companies".to_string()),
        fields: Some(vec!["Name".to_string(), "Market Cap".to_string()]),
        limit: Some(10),
        sort: Some(CompoundSort {
            sorts: Some(vec![SortExpression {
                attribute: "Market Cap".to_string(),
                order: SortOrder::Descending,
            }]),
        }),
        weights: None,
        max_distance: None,
        filters: None,
        vector_query: None,
    }).await.context("Search with sort failed")?;

    if let Some(first) = res.data.first() {
        println!("Most valued tech company: {}, {:?}", first["Name"], first["Market Cap"]);
    }

    // Search with filtering and sorting (trillion dollar companies, ascending)
    let res = client.search_data(&SearchRequest {
        collection: COL.to_string(),
        query: Some("technology companies".to_string()),
        fields: Some(vec!["Name".to_string(), "Market Cap".to_string()]),
        limit: Some(10),
        sort: Some(CompoundSort {
            sorts: Some(vec![SortExpression {
                attribute: "Market Cap".to_string(),
                order: SortOrder::Ascending,
            }]),
        }),
        filters: Some(CompoundFilter {
            and: Some(vec![FilterExpression {
                attribute: Some("Market Cap".to_string()),
                op: Some(FilterOp::GreaterThanOrEqual),
                value: Some(serde_json::json!(1000000000000u64)),
                values: None,
            }]),
        }),
        weights: None,
        max_distance: None,
        vector_query: None,
    }).await.context("Search with filter and sort failed")?;

    if let Some(first) = res.data.first() {
        println!("Least valued trillion dollar tech company: {}, {:?}", first["Name"], first["Market Cap"]);
    }

    Ok(())
}
