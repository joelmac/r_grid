
use crate::schema::{ems_station_id};
//use crate::{ToPSSE};

#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_station_id"]
pub struct Station {
    pub id: u64,
    #[column_name = "ems_station_name"]
    pub name: String,
    #[column_name = "ems_co_id"]
    pub company_id: u64,
}
