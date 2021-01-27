table! {
    charges (charge_id) {
        charge_id -> Varchar,
        shop_domain -> Nullable<Varchar>,
    }
}

table! {
    merchants (shop_domain) {
        shop_domain -> Varchar,
        access_token -> Varchar,
    }
}

joinable!(charges -> merchants (shop_domain));

allow_tables_to_appear_in_same_query!(
    charges,
    merchants,
);
