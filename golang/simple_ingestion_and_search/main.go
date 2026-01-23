package main

import (
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/anvitra-ai/shilp-sdk-go"
)

const col = "us-stocks"
const shilpUrl = "http://localhost:3000"

func main() {
	// Initialize the client
	client := shilp.NewClient(shilpUrl, shilp.WithHTTPClient(&http.Client{
		Timeout: 10 * time.Minute,
	}))
	// you can provide custom http client or use default one by not providing a client at all
	// client := shilp.NewClient(shilpUrl)

	// Check health
	health, err := client.HealthCheck()
	if err != nil {
		log.Fatalf("Health check failed: %v", err)
	}
	fmt.Printf("Health: %v\n", health.Success)

	// check if the collection exist
	exist := checkIfCollectionExists(client)

	// if collection exist, drop it and add the data again
	// change this flag to avoid re-indexing the data
	if !exist {
		dropAndaddData(client)
	}

	searchData(client)
}

func searchData(client *shilp.Client) {
	// let us seach for technology companies
	res, err := client.SearchData(shilp.SearchRequest{
		Collection: col,
		Query:      "technology companies",
		Fields:     []string{"Name", "Market Cap"},
		Limit:      10,
	})
	if err != nil {
		log.Fatalf("Search failed: %v", err)
	}
	fmt.Println("Search Results After Update:", res.Data[0]["Name"], res.Data[0]["Market Cap"])

	// and see who is the most valued based on the market cap
	res, err = client.SearchData(shilp.SearchRequest{
		Collection: col,
		Query:      "technology companies",
		Fields:     []string{"Name", "Market Cap"},
		Limit:      10,
		Sort: shilp.CompoundSort{
			Sorts: []shilp.SortExpression{{"Market Cap", shilp.SortDescending}},
		},
	})
	if err != nil {
		log.Fatalf("Search failed: %v", err)
	}
	fmt.Println("Search Results After Update:", res.Data[0]["Name"], res.Data[0]["Market Cap"])

	// And the least valued among the trillion dollar technology companies
	res, err = client.SearchData(shilp.SearchRequest{
		Collection: col,
		Query:      "technology companies",
		Fields:     []string{"Name", "Market Cap"},
		Limit:      10,
		Sort: shilp.CompoundSort{
			Sorts: []shilp.SortExpression{{"Market Cap", shilp.SortAscending}},
		},
		Filters: shilp.CompoundFilter{
			And: []shilp.FilterExpression{{"Market Cap", shilp.OpGreaterThanOrEqual, 1000000000000, nil}},
		},
	})
	if err != nil {
		log.Fatalf("Search failed: %v", err)
	}
	fmt.Println("Search Results After Update:", res.Data[0]["Name"], res.Data[0]["Market Cap"])
}

func checkIfCollectionExists(client *shilp.Client) bool {
	// List collections
	collections, err := client.ListCollections()
	if err != nil {
		log.Fatalf("Failed to list collections: %v", err)
	}
	fmt.Printf("Collections: %+v\n", collections.Data)
	// Create a collection if doesn't exist, if exists delete it
	exists := false
	for _, c := range collections.Data {
		if c.Name == col {
			exists = true
			break
		}
	}
	fmt.Println("collection exists - ", exists)

	return exists
}

func dropAndaddData(client *shilp.Client) {

	_, err := client.DropCollection(col)
	if err != nil {
		log.Fatalf("Failed to drop existing collection: %v", err)
	}
	_, err = client.AddCollection(shilp.AddCollectionRequest{
		Name:                 col,
		HasMetadataStorage:   true,
		StorageType:          shilp.StorageBackendFile,
		ReferenceStorageType: shilp.StorageBackendFile,
	})
	if err != nil {
		log.Fatalf("Failed to add collection: %v", err)
	}

	// upload the data file
	_, err = client.UploadDataFile("data.csv")
	if err != nil {
		log.Fatalf("Failed to upload the data file: %v", err)
	}
	log.Println("file uploaded successfully - data.csv")

	// begin ingestion
	_, err = client.IngestData(shilp.IngestRequest{
		FilePath:       "data.csv",
		SourceType:     shilp.IngestSourceTypeFile,
		CollectionName: col,

		// ingestion configuration
		KeywordFields:  []string{"Managing Director", "Sector", "Name"},
		MetadataFields: map[string]shilp.AttrType{"Market Cap": shilp.AttrTypeFloat64},
		Fields:         []string{"Description"},
	})
	if err != nil {
		log.Fatalf("error ingesting data. %v", err)
	}
}
