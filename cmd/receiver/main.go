package main

import (
	"fmt"
	"net/http"

	"github.com/go-chi/chi"
	"github.com/go-chi/chi/middleware"
)

func authenticate(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		authorization := w.Header().Get("Authorization")

		if authorization != "xD" {
			http.Error(w, "xDDDDD", http.StatusUnauthorized)
			return
		}

		next.ServeHTTP(w, r)
	})
}

func main() {
	fmt.Println("vim-go")
	r := chi.NewRouter()
	r.Use(middleware.Logger)
	r.Use(authenticate)
	r.Post("/push", func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("xDDD"))
	})
	http.ListenAndServe(":3003", r)
}
