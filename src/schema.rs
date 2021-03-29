table! {
    readings {
        measurement_time_default -> Nullable<Timestamptz>,
        id -> Nullable<Integer>,
        index -> Nullable<Integer>,
        field_description -> Nullable<Text>,
        measurement -> Nullable<Float>,
    }
}

table! {
    locations {
        publication_time -> Nullable<Timestamptz>,
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        latitude -> Nullable<Float>,
        longitude -> Nullable<Float>,
    }
}
