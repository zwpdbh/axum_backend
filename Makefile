diesel_demo_show_post:
	cargo run -p axum_graphql diesel-demo show-post

diesel_demo_create_post:
	cargo run -p axum_graphql diesel-demo create-post --title "my first post" --body "diesel is great"

diesel_demo_update_post:
	cargo run -p axum_graphql diesel-demo update-post --id 1

diesel_demo_select_post:
	cargo run -p axum_graphql diesel-demo select-post --id 3

diesel_demo_delete_post:
	cargo run -p axum_graphql diesel-demo delete-post --target my

sqlx_demo_test: 
	cargo run -p axum_graphql sqlx-demo test

sqlx_demo_bookstore_create:
	cargo run -p axum_graphql sqlx-demo bookstore create 

sqlx_demo_bookstore_update:
	cargo run -p axum_graphql sqlx-demo bookstore update 

sqlx_demo_bookstore_read:
	cargo run -p axum_graphql sqlx-demo bookstore read -v 3

graphql_demo_start: 
	cargo run -p axum_graphql graphql-demo --port 3000
