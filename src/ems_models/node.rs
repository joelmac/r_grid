use crate::schema::{ems_node_id};
use crate::ems_models::station::Station;
use crate::{ToPSSE};
use crate::RGrid;

#[derive(QueryableByName,Queryable,Debug,Identifiable,Associations,Clone)]
#[belongs_to(Station,foreign_key="ems_station_id")]
#[table_name="ems_node_id"]
pub struct Node {
    pub id: u64,
    #[column_name = "ems_node_name"]
    pub name: String,
    pub ems_co_id: u64,
    #[column_name = "ems_station_id"]
    pub station_id: u64,
    #[column_name = "ems_voltage"]
    pub voltage: Option<f32>,
    pub pti_name: Option<String>,
    pub pti_number: u64,
    pub bus_id: u64,
}

impl ToPSSE for Node {

    fn to_psse(&self, _r_grid: &RGrid) -> String {
        let psse_string = format!("{bus_number},'{name}',{basekv},{ide},{gl},{bl},{area},{zone},{vm},{va},{owner}\n",
                                  bus_number=self.pti_number,
                                  name=self.name,
                                  basekv=self.voltage.unwrap_or(0.0),
                                  ide=1,
                                  gl=0.0,
                                  bl=0.0,
                                  area=1,
                                  zone=1,
                                  vm=1.0,
                                  va=0.0,
                                  owner=1);
        psse_string.to_string()

    }
}
