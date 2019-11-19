use crate::schema::{ems_transformer_id};
//use crate::{ToPSSE};



#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_transformer_id"]
pub struct Transformer {
    pub id: u32,
    #[column_name = "ems_xf_name"]
    pub xf_name : Option<String>,
    #[column_name = "ems_xfmr_name"]
    pub xfmr_name : Option<String>,
    pub ems_from_node : u64,
    pub ems_to_node : u64,
    pub ems_reg_node : Option<u64>,
    #[column_name = "R"]
    pub r : Option<f32>,
    #[column_name = "X"]
    pub x : Option<f32>,
    pub idc_from_bus : u64,
    pub idc_to_bus : u64,
    pub idc_ref_id : Option<u64>,
}


