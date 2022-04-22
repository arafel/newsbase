table! {
    articles (id) {
        id -> Int4,
        newsgroup_id -> Int4,
        server_id -> Int4,
        author -> Text,
        subject -> Text,
        date_sent -> Text,
    }
}

table! {
    newsgroups (id) {
        id -> Int4,
        name -> Text,
        low -> Int4,
        high -> Int4,
    }
}

joinable!(articles -> newsgroups (newsgroup_id));

allow_tables_to_appear_in_same_query!(
    articles,
    newsgroups,
);
