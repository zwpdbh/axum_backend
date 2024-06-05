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