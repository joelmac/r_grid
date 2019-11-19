pub mod schema;
pub mod ems_models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use id_arena::{Arena, Id};
use ems_models::*;

pub fn establish_connection() -> MysqlConnection {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connection to {}", database_url))
}

#[derive(Debug)]
pub struct RGrid {
    pub name: String,
    station_arena: Arena<ems_models::Station>,
    node_arena: Arena<ems_models::Node>,
    line_arena: Arena<ems_models::Line>,
    zbr_arena: Arena<ems_models::Zbr>,
    company_arena: Arena<ems_models::Company>,
    transformer_arena: Arena<ems_models::Transformer>,
    generator_arena: Arena<ems_models::Generator>,
    load_arena: Arena<ems_models::Load>,
    station_id_converter: HashMap<u64, Id<ems_models::Station>>,
    node_id_converter: HashMap<u64, Id<ems_models::Node>>,
    line_id_converter: HashMap<u32, Id<ems_models::Line>>,
    zbr_id_converter: HashMap<u32, Id<ems_models::Zbr>>,
    company_id_converter: HashMap<u64, Id<ems_models::Company>>,
    transformer_id_converter: HashMap<u32, Id<ems_models::Transformer>>,
    generator_id_converter: HashMap<u32, Id<ems_models::Generator>>,
    load_id_converter: HashMap<u64, Id<ems_models::Load>>,
    node_key_lookup: HashMap<(String, String),u64>,
    station_key_lookup: HashMap<String,u64>,
    load_key_lookup: HashMap<(u64,String), u64>,
}

impl RGrid {

    pub fn new(name: String) -> Self {
        RGrid {
            name: name,
            node_arena: Arena::<ems_models::Node>::new(),
            node_id_converter: HashMap::<u64,Id<ems_models::Node>>::new(),
            zbr_arena: Arena::<ems_models::Zbr>::new(),
            zbr_id_converter: HashMap::<u32, Id<ems_models::Zbr>>::new(),
            station_arena: Arena::<ems_models::Station>::new(),
            station_id_converter: HashMap::<u64,Id<ems_models::Station>>::new(),
            line_arena: Arena::<ems_models::Line>::new(),
            line_id_converter: HashMap::<u32,Id<ems_models::Line>>::new(),
            transformer_arena: Arena::<ems_models::Transformer>::new(),
            transformer_id_converter: HashMap::<u32,Id<ems_models::Transformer>>::new(),
            generator_arena: Arena::<ems_models::Generator>::new(),
            generator_id_converter: HashMap::<u32,Id<ems_models::Generator>>::new(),
            company_arena: Arena::<ems_models::Company>::new(),
            company_id_converter: HashMap::<u64,Id<ems_models::Company>>::new(),
            load_arena: Arena::<ems_models::Load>::new(),
            load_id_converter: HashMap::<u64,Id<ems_models::Load>>::new(),
            node_key_lookup: HashMap::<(String, String), u64>::new(),
            station_key_lookup: HashMap::<String, u64>::new(),
            load_key_lookup: HashMap::<(u64,String), u64>::new(),
        }
    }

    pub fn initialize_station_keys(&mut self) {
        for (_,station) in self.station_arena.iter() {
            self.station_key_lookup.insert(station.name.clone(),station.id);
        }
    }

    pub fn initialize_load_keys(&mut self) {
        for (_,load) in self.load_arena.iter() {
            let station_id = self.get_node(load.node_id).unwrap().station_id;
            self.load_key_lookup.insert((station_id,load.name.as_ref().unwrap().to_string()),load.id);
        }
    }

    pub fn initialize_node_keys(&mut self) {
        for (_,node) in self.node_arena.iter() {
            let station_name = match self.get_station(node.station_id) {
                Some(station) => station.name.clone(),
                _ => continue,
            };
            self.node_key_lookup.insert((station_name,node.name.clone()),node.id);
        }
    }

    pub fn add_generator(&mut self, generator: ems_models::Generator) {
        let generator_id = generator.id;
        let arena_id = self.generator_arena.alloc(generator);
        self.generator_id_converter.insert(generator_id,arena_id);
    }

    pub fn add_company(&mut self, company: ems_models::Company) {
        let company_id = company.id;
        let arena_id = self.company_arena.alloc(company);
        self.company_id_converter.insert(company_id,arena_id);
    }

    pub fn add_station(&mut self, station: ems_models::Station) {
        let station_id = station.id;
        let arena_id = self.station_arena.alloc(station);
        self.station_id_converter.insert(station_id,arena_id);
    }

    pub fn add_node(&mut self, node: ems_models::Node) {
        let node_id = node.id;
        let arena_id = self.node_arena.alloc(node);
        self.node_id_converter.insert(node_id,arena_id);
    }

    pub fn add_line(&mut self, line: ems_models::Line) {
        let line_id = line.id;
        let arena_id = self.line_arena.alloc(line);
        self.line_id_converter.insert(line_id,arena_id);
    }

    pub fn add_load(&mut self, ems_load: ems_models::Load) {
        let load_id = ems_load.id;
        let arena_id = self.load_arena.alloc(ems_load);
        self.load_id_converter.insert(load_id,arena_id);
    }

    pub fn add_zbr(&mut self, zbr: ems_models::Zbr) {
        let zbr_id = zbr.id;
        let arena_id = self.zbr_arena.alloc(zbr);
        self.zbr_id_converter.insert(zbr_id,arena_id);
    }

    pub fn add_transformer(&mut self, transformer: ems_models::Transformer) {
        let transformer_id = transformer.id;
        let arena_id = self.transformer_arena.alloc(transformer);
        self.transformer_id_converter.insert(transformer_id,arena_id);
    }

    pub fn get_station(&self, station_id: u64) -> Option<&ems_models::Station> {
        let arena_id = self.station_id_converter.get(&station_id).expect("ID was missing from arena!");
        self.station_arena.get(*arena_id) 
    }

    pub fn get_generator(&self, generator_id: u32) -> Option<&ems_models::Generator> {
        let generator_id = self.generator_id_converter.get(&generator_id).expect("Generator ID was mission from arena!");
        self.generator_arena.get(*generator_id)
    }

    pub fn get_company(&self, company_id: u64) -> Option<&ems_models::Company> {
        let company_id = self.company_id_converter.get(&company_id).expect("company ID was mission from arena!");
        self.company_arena.get(*company_id)
    }

    pub fn get_line(&self, line_id: u32) -> Option<&ems_models::Line> {
        let arena_id = self.line_id_converter.get(&line_id).expect("ID was missing from arena!");
        self.line_arena.get(*arena_id) 
    }

    pub fn get_zbr(&self, zbr_id: u32) -> Option<&ems_models::Zbr> {
        let arena_id = self.zbr_id_converter.get(&zbr_id).expect("ID was missing from arena!");
        self.zbr_arena.get(*arena_id) 
    }

    pub fn get_transformer(&self, transformer_id: u32) -> Option<&ems_models::Transformer> {
        let arena_id = self.transformer_id_converter.get(&transformer_id).expect("ID was missing from arena!");
        self.transformer_arena.get(*arena_id)
    }


    pub fn get_node(&self, node_id: u64) -> Option<&ems_models::Node> {
        let arena_id_option = self.node_id_converter.get(&node_id);
        match arena_id_option {
            Some(arena_id) => self.node_arena.get(*arena_id),
            _ => {
                println!("Node ID {} was missing from arena!",node_id);
                None
            },
        }
    }

    pub fn get_load(&self, load_id: u64) -> Option<&ems_models::Load> {
        let arena_id_option = self.load_id_converter.get(&load_id);
        match arena_id_option {
            Some(arena_id) => self.load_arena.get(*arena_id),
            _ => {
                println!("Node ID {} was missing from arena!",load_id);
                None
            },
        }
    }

    pub fn get_node_by_key(&self,station_name: String, node_name: String) -> Option<&ems_models::Node> {
        //first find the company
        let node_id = self.node_key_lookup.get(&(station_name,node_name));
        match node_id {
            Some(id) => self.get_node(*id),
            _ => None,
        }
    }

    pub fn get_station_by_name(&self, station_name: String) -> Option<&ems_models::Station> {
        let station_id = self.station_key_lookup.get(&station_name);
        match station_id {
            Some(id) => self.get_station(*id),
            _ => None,
        }
    }
    
    pub fn get_load_by_key(&self,station_id: u64, load_name: String) -> Option<&ems_models::Load> {
        //first find the company
        let load_id = self.load_key_lookup.get(&(station_id,load_name.clone()));
        match load_id {
            Some(id) => self.get_load(*id),
            _ => None,
        }
    }

}

pub trait ToPSSE {

    fn to_psse(&self,r_grid: &RGrid) -> String;

}

impl RGrid {

    pub fn initialize_lines(&mut self) {
        use self::schema::ems_line_id::dsl::ems_line_id as ems_line_id_table;
        let connection = establish_connection();
        let lines = ems_line_id_table
            .load::<Line>(&connection)
            .expect("Error loading nodes!");
        for line in lines {
            self.add_line(line);
        }
    }

    pub fn initialize_transformers(&mut self) {
        use self::schema::ems_transformer_id::dsl::ems_transformer_id as ems_transformer_id_table;
        let connection = establish_connection();
        let transformers = ems_transformer_id_table
            .load::<Transformer>(&connection)
            .expect("Error loading transformers!");
        for transformer in transformers {
            self.add_transformer(transformer);
        }
    }

    pub fn initialize_nodes(&mut self) {
        use self::schema::ems_node_id::dsl::ems_node_id as ems_node_id_table;
        let connection = establish_connection();
        let nodes = ems_node_id_table
            .load::<Node>(&connection)
            .expect("Error loading nodes!");
        for node in nodes {
            self.add_node(node);
        }
    }

    pub fn initialize_zbr(&mut self) {
        use self::schema::ems_zbr_id::dsl::ems_zbr_id as ems_zbr_id_table;
        let connection = establish_connection();
        let zbrs = ems_zbr_id_table
            .load::<Zbr>(&connection)
            .expect("Error loading nodes!");
        for zbr in zbrs {
            self.add_zbr(zbr);
        }
    }

    pub fn initialize_stations(&mut self) {
        use self::schema::ems_station_id::dsl::ems_station_id as ems_station_id_table;
        let connection = establish_connection();
        let stations = ems_station_id_table
            .load::<Station>(&connection)
            .expect("Error loading nodes!");
        for station in stations {
            self.add_station(station);
        }
    }

    pub fn initialize_generators(&mut self) {
        use self::schema::ems_generator_id::dsl::ems_generator_id as ems_generator_id_table;
        let connection = establish_connection();
        let generators = ems_generator_id_table
            .load::<Generator>(&connection)
            .expect("Error loading nodes!");
        for generator in generators {
            self.add_generator(generator);
        }
    }

    pub fn initialize_companies(&mut self) {
        use self::schema::ems_co_id::dsl::ems_co_id as ems_co_id_table;
        let connection = establish_connection();
        let companies = ems_co_id_table
            .load::<Company>(&connection)
            .expect("Error loading nodes!");
        for company in companies {
            self.add_company(company);
        }
    }

    pub fn initialize_loads(&mut self) {
        use self::schema::ems_load_id::dsl::ems_load_id as ems_load_id_table;
        let connection = establish_connection();
        let loads = ems_load_id_table
            .load::<Load>(&connection)
            .expect("Error loading nodes!");
        for load in loads {
            self.add_load(load);
        }
    }

    pub fn to_psse(&self) -> String {
        let mut result = "".to_string();
        //initially just work with nodes.
        let mut bus_data  = "".to_string();
        for (_id, node) in self.node_arena.iter() {
            bus_data.push_str(&node.to_psse(&self));
        }
        // creating bus data goes here
        bus_data.push_str("0 / END OF BUS DATA, BEGIN LOAD DATA\n");
        let mut load_data = "".to_string();
        // creating load data goes here
        load_data.push_str("0 / END OF LOAD DATA, BEGIN GENERATOR DATA\n");
        let mut generator_data = "".to_string();
        // creating generator data goes here
        generator_data.push_str("0 / END OF GENERATOR DATA, BEGIN BRANCH DATA\n");
        let mut branch_data = "".to_string();
        for (_id, line) in self.line_arena.iter() {
            branch_data.push_str(&line.to_psse(&self));
        }
        // creating branch data goes here
        branch_data.push_str("0 / END OF BRANCH DATA, BEGIN TRANSFORMER DATA\n");
        let mut transformer_data = "".to_string();
        // creating transformer data goes here
        transformer_data.push_str("0 / END OF TRANSFORMER DATA, BEGIN AREA DATA\n");
        let mut area_data = "".to_string();
        // creating area data goes here
        area_data.push_str("0 / END OF AREA DATA, BEGIN TWO-TERMINAL DC DATA\n");
        let mut dc_data = "".to_string();
        // creating two-terminal dc data goes here
        dc_data.push_str("0 / END OF TWO-TERMINAL DC DATA, BEGIN VOLTAGE SOURCE CONVERTER DATA\n");
        let mut voltage_converter_data = "".to_string();
        // creating voltage source converter data goes here
        voltage_converter_data.push_str("0 / END OF VOLTAGE SOURCE CONVERTER DATA, BEGIN SWITCHED SHUNT DATA\n");
        let mut switched_shunt_data = "".to_string();
        // creating switched shunt data goes here
        switched_shunt_data.push_str("0 / END OF SWITCHED SHUNT DATA, BEGIN IMPEDANCE CORRECTION DATA\n");
        let mut impedance_correction_data = "".to_string();
        // creating impedance correction data goes here
        impedance_correction_data.push_str("0 / END OF IMPEDANCE CORRECTION DATA, BEGIN MULTI-TERMINAL DC DATA\n");
        let mut multi_terminal_dc_data = "".to_string();
        // creating multi terminal dc data goes here
        multi_terminal_dc_data.push_str("0 / END OF MULTI-TERMINAL DC DATA, BEGIN MULTI-SECTION LINE DATA\n");
        let mut multi_section_line_data = "".to_string();
        // creating multi section line data goes here
        multi_section_line_data.push_str("0 / END OF MULTI-SECTION LINE DATA, BEGIN ZONE DATA\n");
        let mut zone_data = "".to_string();
        // creating zone data goes here
        zone_data.push_str("0 / END OF ZONE DATA, BEGIN INTER-AREA TRANSFER DATA\n");
        let mut inter_area_transfer_data = "".to_string();
        // creating inter-area transfer data goes here
        inter_area_transfer_data.push_str("0 / END OF INTER-AREA TRANSFER DATA, BEGIN OWNER DATA\n");
        let mut owner_data = "".to_string();
        // creating owner data goes here
        owner_data.push_str("0 / END OF OWNER DATA, BEGIN FACTS CONTROL DEVICE DATA\n");
        let mut facts_control_device_data = "".to_string();
        // creating facts control device data goes here
        facts_control_device_data.push_str("0 / END OF FACTS CONTROL DEVICE DATA\n");



        result.push_str(&bus_data);
        result.push_str(&load_data);
        result.push_str(&generator_data);
        result.push_str(&branch_data);
        result.push_str(&transformer_data);
        result.push_str(&area_data);
        result.push_str(&dc_data);
        result.push_str(&voltage_converter_data);
        result.push_str(&switched_shunt_data);
        result.push_str(&impedance_correction_data);
        result.push_str(&multi_terminal_dc_data);
        result.push_str(&multi_section_line_data);
        result.push_str(&zone_data);
        result.push_str(&inter_area_transfer_data);
        result.push_str(&owner_data);
        result.push_str(&facts_control_device_data);
        return result;

    }

}

#[cfg(test)]
mod tests;
