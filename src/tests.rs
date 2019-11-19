#[cfg(test)]
mod tests {
    use ems_models::*;
    use crate::*;

    #[test]
    fn test_initialize_nodes_from_mysql() {
        let mut rg = RGrid::new("MISO".to_string());
        rg.initialize_nodes();
        rg.get_node(9501).expect("Missing node!");
    }

    #[test]
    fn test_initialize_lines_from_mysql() {
        let mut rg = RGrid::new("MISO".to_string());
        rg.initialize_lines();
        rg.get_line(6166).expect("Missing node!");
    }

    #[test]
    fn test_initialize_transformers_from_mysql() {
        let mut rg = RGrid::new("MISO".to_string());
        rg.initialize_transformers();
        rg.get_transformer(43792).expect("Missing node!");
    }

    #[test]
    fn test_initialize_zbr_from_mysql() {
        let mut rg = RGrid::new("MISO".to_string());
        rg.initialize_zbr();
        rg.get_zbr(3049204).expect("Missing node!");
    }

    #[test]
    fn test_initialize_generators_from_mysql() {
        let mut rg = RGrid::new("MISO".to_string());
        rg.initialize_generators();
        rg.get_generator(530536).expect("Missing node!");
    }

    #[test]
    fn test_initialize_station_from_mysql() {
        let mut rg = RGrid::new("MISO".to_string());
        rg.initialize_stations();
        rg.get_station(3040).expect("Missing station!");
    }

    #[test]
    fn test_initialize_company_from_mysql() {
        let mut rg = RGrid::new("MISO".to_string());
        rg.initialize_companies();
        rg.get_company(58257197).expect("Missing company!");
    }

    #[test]
    fn test_get_node_by_key() {
        let mut rg = RGrid::new("MISO".to_string());
        rg.initialize_stations();
        rg.initialize_nodes();
        rg.initialize_node_keys();
        let node = rg.get_node_by_key("SUMNER".to_string(),"4".to_string());
        assert_eq!(node.expect("No node returned!").id,9501);
    }


}
