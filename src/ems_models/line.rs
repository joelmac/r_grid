use crate::schema::ems_line_id;
use crate::{ToPSSE};
use crate::RGrid;

#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_line_id"]
pub struct Line {
    pub id: u32,
    #[column_name = "ems_line_name"]
    pub name: Option<String>,
    #[column_name = "ems_segment"]
    pub segment: Option<String>,
    #[column_name = "ems_from_node"]
    pub from_node: u64,
    #[column_name = "ems_to_node"]
    pub to_node: u64,
    pub r: Option<f32>,
    pub x: Option<f32>,
    pub bch: Option<f32>,
    pub idc_from_bus: Option<u32>,
    pub idc_to_bus: Option<u32>,
    #[column_name = "idc_ref_id"]
    pub idc_ref: Option<u32>,
}

impl ToPSSE for Line {

    fn to_psse(&self, r_grid: &RGrid) -> String {
        let from_bus = match r_grid.get_node(self.from_node) {
                    Some(node) => node.pti_number,
                    _ => {
                        println!("Could not find referenced node for line {:?}!",self);
                        0
                    }
        };
        let to_bus = r_grid.get_node(self.to_node).expect("Could not find refenced node!").pti_number;
        let psse_string = format!("{from_bus},{to_bus},{circuit_identifier},{resistance},{reactance},{susceptance},{ratea},{rateb},{ratec},{gi},{bi},{gj},{bj},{status},{length},{owner_number},{owner_fraction}\n",
                                  from_bus = from_bus,
                                  to_bus = to_bus,
                                  circuit_identifier = self.segment.as_ref().unwrap_or(&"1".to_string()),
                                  resistance = self.r.unwrap_or(0.0),
                                  reactance = self.x.unwrap_or(0.0),
                                  susceptance = self.bch.unwrap_or(0.0),
                                  ratea = 100.0,
                                  rateb = 100.0,
                                  ratec = 100.0,
                                  gi = 0.0,
                                  bi = 0.0,
                                  gj = 0.0,
                                  bj = 0.0,
                                  status = 1,
                                  length = 0.0,
                                  owner_number = 1,
                                  owner_fraction = 1.0);
        psse_string.to_string()
    }
}

