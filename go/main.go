package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

const PORT_DEFAULT = "8080"

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "Hello world!")
	})

	port := os.Getenv("PORT")
	if port == "" {
		port = PORT_DEFAULT
	}

	http.ListenAndServe(":"+port, nil)
}
