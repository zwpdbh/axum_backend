diesel_demo_show_post:
	cargo run -p axum_graphql diesel-demo show-post

diesel_demo_create_post:
	cargo run -p axum_graphql diesel-demo create-post --title "my first post" --body "diesel is great"