use crate::schema::ems_load_id;
//use crate::{ToPSSE};

#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_load_id"]
pub struct Load {
    pub id: u64,
    #[column_name = "ems_load_name"]
    pub name: Option<String>,
    #[column_name = "ems_node_id"]
    pub node_id: u64,
    pub capacity: Option<f32>,
    pub mvar: Option<f32>,
    pub parfrac: Option<f32>,
    pub pf: Option<f32>,
}
