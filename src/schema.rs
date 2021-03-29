table! {
    readings {
        measurement_time_default -> Nullable<Timestamptz>,
        id -> Nullable<Integer>,
        index -> Nullable<Integer>,
        field_description -> Nullable<Text>,
        measurement -> Nullable<Float>,
    }
}
