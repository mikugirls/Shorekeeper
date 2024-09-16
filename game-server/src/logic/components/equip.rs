use crate::logic::ecs::component::Component;
use shorekeeper_protocol::entity_component_pb::ComponentPb;
use shorekeeper_protocol::{EntityComponentPb, EquipComponentPb};

pub struct Equip {
    pub weapon_id: i32,
    pub weapon_breach_level: i32,
}

impl Component for Equip {
    fn set_pb_data(&self, pb: &mut shorekeeper_protocol::EntityPb) {
        pb.component_pbs.push(EntityComponentPb {
            component_pb: Some(ComponentPb::EquipComponent(EquipComponentPb {
                weapon_id: self.weapon_id,
                weapon_breach_level: self.weapon_breach_level,
            })),
        })
    }
}