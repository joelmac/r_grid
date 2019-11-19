use crate::schema::ems_co_id;
//use crate::{ToPSSE};

#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_co_id"]
pub struct Company {
    pub id: u64,
    #[column_name = "ems_co_name"]
    pub name : Option<String>,
}
