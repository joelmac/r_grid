use crate::schema::ems_generator_id;
use crate::{RGrid, ToPSSE};

#[derive(QueryableByName,Queryable,Debug,Identifiable,Clone)]
#[table_name="ems_generator_id"]
pub struct Generator {
    pub id: u32,
    pub capacity: Option<f32>,
    pub min_capacity: Option<f32>,
    pub min_mvar: Option<f32>,
    pub max_mvar: f32,
    pub power_factor: Option<f32>,
    pub idc_bus_id: Option<u64>,
    #[column_name ="ems_unit_name"]
    pub name: Option<String>,
    #[column_name = "ems_node_id"]
    pub node: u64,
    #[column_name = "ems_reg_node"]
    pub reg_node: u64,
    pub target_voltage: Option<f32>,
}

impl ToPSSE for Generator {
    fn to_psse(&self, r_grid: &RGrid) -> String {

        let bus_number = r_grid.get_node(self.node).unwrap().pti_number;
        let target_bus_number =r_grid.get_node(bus_number).unwrap().pti_number;

        let psse_string = format!("{bus_number},{ID},{PG},{QG},{QT},{QB},{VS},{IREG},{MBASE},{ZR},{ZX},{RT},{XT},{GTAP},{STAT},{RMPCT},{PT},{PB},{O},{F}\n",
                bus_number = bus_number,
                ID=self.name.as_ref().unwrap_or(&"G1".to_string()),
                PG=0.0,
                QG=0.0,
                QT=self.max_mvar,
                QB=self.min_mvar.unwrap_or(-9999.0),
                VS=1.0,
                IREG=target_bus_number,
                MBASE="",
                ZR=0.0,
                ZX=0.0,
                RT=0.0,
                XT=0.0,
                GTAP=1.0,
                STAT=1,
                RMPCT=100.0,
                PT=self.capacity.unwrap_or(0.0),
                PB=self.min_capacity.unwrap_or(0.0),
                O=1,
                F=1.0);

        psse_string.to_string()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_generator() {
         let g = Generator {
            id: 1,
            capacity: Some(1.0),
            min_capacity: Some(1.0),
            min_mvar: Some(1.0),
            max_mvar: 1.0,
            power_factor: Some(1.0),
            idc_bus_id: Some(1),
            name: Some("My test generator".to_string()),
            node: 1,
            reg_node: 1,
            target_voltage: Some(1.0),
        };
        assert_eq!(g.id,1);

    }

}
