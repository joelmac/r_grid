use crate::schema::ems_capacitor_id;
//use crate::{ToPSSE};

#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_capacitor_id"]
pub struct Capacitor {
    pub id: u32,
    #[column_name = "ems_capacitor_name"]
    pub name : String,
    #[column_name = "ems_node_id"]
    pub node : u32,
    pub ems_reg_node : Option<u32>,
    pub target_voltage : Option<f32>,
}
