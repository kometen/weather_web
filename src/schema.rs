table! {
    readings {
        measurement_time_default -> Nullable<Timestamptz>,
        id -> Nullable<Integer>,
        data -> Nullable<Jsonb>,
    }
}

table! {
    locations {
        publication_time -> Nullable<Timestamptz>,
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
    }
}
