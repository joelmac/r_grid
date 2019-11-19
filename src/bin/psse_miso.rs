extern crate r_grid;
extern crate diesel;
extern crate id_arena;

use r_grid::*;
use self::ems_models::*;
use self::diesel::prelude::*;

fn main() {
    use r_grid::schema::ems_station_id::dsl::ems_station_id as ems_station_id_table;
    use r_grid::schema::ems_breaker_id::dsl::ems_breaker_id as ems_breaker_id_table;
    use r_grid::schema::ems_breaker_type_id::dsl::ems_breaker_type_id as ems_breaker_type_id_table;
    use r_grid::schema::ems_node_id::dsl::ems_node_id as ems_node_id_table;
    use r_grid::schema::ems_line_id::dsl::ems_line_id as ems_line_id_table;
    use r_grid::schema::ems_generator_id::dsl::ems_generator_id as ems_generator_id_table;

    let mut rg = RGrid::new("MISO".to_string()); 

    rg.initialize_generators();
    rg.initialize_nodes();
    rg.initialize_stations();
    rg.initialize_transformers();

    println!("Elements added added. Serializing to RAW...");
    let raw = rg.to_psse();
    println!("{}",raw);
}
