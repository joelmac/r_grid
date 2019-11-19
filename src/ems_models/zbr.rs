use crate::schema::ems_zbr_id;
//use crate::{ToPSSE};

#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_zbr_id"]
pub struct Zbr {
    pub id: u32,
    #[column_name = "ems_zbr_line_name"]
    pub name: Option<String>,
    pub ems_from_node: u64,
    pub ems_to_node: u64,
    pub ems_segment: Option<String>,
    pub idc_from_bus: Option<u64>,
    pub idc_to_bus: Option<u64>,
}
