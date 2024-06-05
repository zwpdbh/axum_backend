// @generated automatically by Diesel CLI.

diesel::table! {
    book (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        isbn -> Varchar,
        metadata -> Nullable<Jsonb>,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        description -> Nullable<Varchar>,
        name -> Varchar,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    item_tags (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
        price -> Numeric,
        added_on -> Date,
        category_id -> Nullable<Int4>,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    items_taggings (item_id, item_tag_id) {
        item_id -> Int4,
        item_tag_id -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        customer_number -> Int4,
        items -> Nullable<Jsonb>,
        ordered_at -> Timestamptz,
        state -> Varchar,
        inserted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::table! {
    todos (id) {
        id -> Int8,
        description -> Text,
        done -> Bool,
    }
}

diesel::joinable!(items -> categories (category_id));
diesel::joinable!(items_taggings -> item_tags (item_tag_id));
diesel::joinable!(items_taggings -> items (item_id));

diesel::allow_tables_to_appear_in_same_query!(
    book,
    categories,
    item_tags,
    items,
    items_taggings,
    orders,
    posts,
    todos,
);
