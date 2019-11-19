use crate::schema::{ems_breaker_id, ems_breaker_type_id};
//use crate::{ToPSSE};

#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_breaker_id"]
pub struct Breaker {
    pub id: u32,
    #[column_name="ems_breaker_name"]
    pub name: Option<String>,
    #[column_name="ems_from_node"]
    pub from_node: u32,
    #[column_name="ems_to_node"]
    pub to_node: u32,
    #[column_name="ems_breaker_type_id"]
    pub breaker_type_id: u32,
}


#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_breaker_type_id"]
pub struct BreakerType {
    pub id: u32,
    #[column_name="ems_breaker_type_short_name"]
    pub name: Option<String>,
    #[column_name="ems_breaker_type_long_name"]
    pub long_name: Option<String>,
}
