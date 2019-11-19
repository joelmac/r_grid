table! {
    ems_breaker_id (id) {
        #[sql_name = "ems_breaker_id"]
        id -> Unsigned<Integer>,
        ems_breaker_name -> Nullable<Varchar>,
        ems_from_node -> Unsigned<Bigint>,
        ems_to_node -> Unsigned<Bigint>,
        ems_breaker_type_id -> Unsigned<Integer>,
    }
}

table! {
    ems_breaker_type_id (id) {
        #[sql_name = "ems_breaker_type_id"]
        id -> Unsigned<Integer>,
        ems_breaker_type_short_name -> Nullable<Varchar>,
        ems_breaker_type_long_name -> Nullable<Varchar>,
    }
}

table! {
    ems_capacitor_id (id) {
        #[sql_name = "ems_capacitor_id"]
        id -> Unsigned<Integer>,
        ems_capacitor_name -> Nullable<Varchar>,
        ems_node_id -> Unsigned<Bigint>,
        ems_reg_node -> Nullable<Unsigned<Integer>>,
        target_voltage -> Nullable<Float>,
    }
}

table! {
    ems_co_id (id) {
        #[sql_name = "ems_co_id"]
        id -> Unsigned<Bigint>,
        ems_co_name -> Nullable<Varchar>,
    }
}

table! {
    ems_generator_id (id) {
        #[sql_name = "ems_generator_id"]
        id -> Unsigned<Integer>,
        capacity -> Nullable<Float>,
        min_capacity -> Nullable<Float>,
        min_mvar -> Nullable<Float>,
        max_mvar -> Float,
        power_factor -> Nullable<Float>,
        idc_bus_id -> Nullable<Unsigned<Bigint>>,
        ems_unit_name -> Nullable<Varchar>,
        ems_node_id -> Unsigned<Bigint>,
        ems_reg_node -> Unsigned<Bigint>,
        target_voltage -> Nullable<Float>,
    }
}

table! {
    ems_line_id (id) {
        #[sql_name = "ems_line_id"]
        id -> Unsigned<Integer>,
        ems_line_name -> Nullable<Varchar>,
        ems_segment -> Nullable<Varchar>,
        ems_from_node -> Unsigned<Bigint>,
        ems_to_node -> Unsigned<Bigint>,
        r -> Nullable<Float>,
        x -> Nullable<Float>,
        bch -> Nullable<Float>,
        idc_from_bus -> Nullable<Unsigned<Integer>>,
        idc_to_bus -> Nullable<Unsigned<Integer>>,
        idc_ref_id -> Nullable<Unsigned<Integer>>,
    }
}

table! {
    ems_load_id (id) {
        #[sql_name = "ems_load_id"]
        id -> Unsigned<Bigint>,
        ems_load_name -> Nullable<Varchar>,
        ems_node_id -> Unsigned<Bigint>,
        capacity -> Nullable<Float>,
        mvar -> Nullable<Float>,
        parfrac -> Nullable<Float>,
        pf -> Nullable<Float>,
    }
}

table! {
    ems_node_id (id) {
        #[sql_name = "ems_node_id"]
        id -> Unsigned<Bigint>,
        ems_node_name -> Varchar,
        ems_co_id -> Unsigned<Bigint>,
        ems_station_id -> Unsigned<Bigint>,
        ems_voltage -> Nullable<Float>,
        pti_name -> Nullable<Varchar>,
        pti_number -> Unsigned<Bigint>,
        bus_id -> Unsigned<Bigint>,
    }
}

table! {
    ems_station_id (id) {
        #[sql_name = "ems_station_id"]
        id -> Unsigned<Bigint>,
        ems_station_name -> Varchar,
        ems_co_id -> Unsigned<Bigint>,
    }
}

table! {
    ems_transformer_id (id) {
        #[sql_name = "ems_transformer_id"]
        id -> Unsigned<Integer>,
        ems_xf_name -> Nullable<Varchar>,
        ems_xfmr_name -> Nullable<Varchar>,
        ems_from_node -> Unsigned<Bigint>,
        ems_to_node -> Unsigned<Bigint>,
        ems_reg_node -> Nullable<Unsigned<Bigint>>,
        R -> Nullable<Float>,
        X -> Nullable<Float>,
        idc_from_bus -> Unsigned<Bigint>,
        idc_to_bus -> Unsigned<Bigint>,
        idc_ref_id -> Nullable<Unsigned<Bigint>>,
    }
}

table! {
    ems_zbr_id (id) {
        #[sql_name = "ems_zbr_id"]
        id -> Unsigned<Integer>,
        ems_zbr_line_name -> Nullable<Varchar>,
        ems_from_node -> Unsigned<Bigint>,
        ems_to_node -> Unsigned<Bigint>,
        ems_segment -> Nullable<Varchar>,
        idc_from_bus -> Nullable<Unsigned<Bigint>>,
        idc_to_bus -> Nullable<Unsigned<Bigint>>,
    }
}

allow_tables_to_appear_in_same_query!(
    ems_breaker_id,
    ems_breaker_type_id,
    ems_capacitor_id,
    ems_co_id,
    ems_generator_id,
    ems_line_id,
    ems_load_id,
    ems_node_id,
    ems_station_id,
    ems_transformer_id,
    ems_zbr_id,
);
