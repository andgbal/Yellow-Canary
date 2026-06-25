pub enum EntitiyType{
    StillLife,
    Mold,
}

pub struct Entitiy {
    pub id: u32,
    pub Entitiy_type: EntitiyType,
    pub aggression: u32,
    pub speed: u32,
}

impl Entity {
    pub fn new(id: u32, e_type: EntityType) -> Self {
        match e_type {
            EntityType::StillLife => Self { id, entity_type: e_type, aggression: 5, speed: 3 },
            EntityType::Mold => Self { id, entity_type: e_type, aggression: 70, speed: 15 },
        }
    }
}