package main

import (
    "fmt"
    "log"
    "net/http"
)

func homePage(w http.ResponseWriter, r *http.Request) {
    w.Header().Set("Access-Control-Allow-Origin", "*")
    w.Header().Set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
    w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

    w.Header().Set("Content-Type", "application/json; charset=utf-8")

    jsonData := `{"message": "Is this working?"}`
    fmt.Fprintf(w, jsonData)
    fmt.Println("Endpoint Hit: homePage")
}

func handleRequests() {
    http.HandleFunc("/api/home", homePage)
    log.Fatal(http.ListenAndServe(":3001", nil))
}

func main() {
    handleRequests()
}
