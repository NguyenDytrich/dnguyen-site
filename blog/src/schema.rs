table! {
    blog_posts (id) {
        id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
        published_at -> Nullable<Timestamptz>,
        publish_state -> Varchar,
        body -> Nullable<Text>,
        title -> Varchar,
        slug -> Varchar,
    }
}
