type NodeId = u64;
type EdgeId = u64;

#[derive(Debug, Clone)] // Allows us to print and duplicate items if needed
pub enum Item {
    /// A physical anchor. The ID matches the corporate serial number.
    Beacon { serial_id: u32 },
    
    /// Uncovers the hidden 'encounter_rate' of a room.
    Sensor,
    
    /// Powers the Explorer's flashlight/radio. Drains every tick.
    Battery { charge: u32 },
}

// Attach methods directly to the Enum
impl Item {
    /// Returns the physical weight of the item.
    pub fn weight(&self) -> u32 {
        match self {
            Item::Beacon { .. } => 5,  // Beacons are heavy metal anchors
            Item::Sensor => 15,        // Sensors are bulky corporate hardware
            Item::Battery { .. } => 2, // Batteries are light
        }
    }
}

// Note: ExplorerState variants should be PascalCase (standBy -> StandBy)
pub enum ExplorerState {
    StandBy,
    Mapping,
    Fleeing { panic_level: u32 },
    Dead,
}

pub struct Explorer {
    pub id: u32,
    pub node_location: NodeId, // Ensure NodeId is defined or imported
    pub inventory: Vec<Item>,
    pub state: ExplorerState,
}

impl Explorer {
    // 1. Return type should be 'Self', not 'self' (lowercase)
    pub fn new(id: u32, node_location: NodeId, loadout: Vec<Item>) -> Self {
        Self {
            id,
            node_location,
            inventory: loadout,
            state: ExplorerState::StandBy,
        }
    }

    // 2. Use '&self' (lowercase), not '&Self' (uppercase)
    pub fn current_payload(&self) -> u32 {
        // 3. Removed semicolon at the end so it returns the value
        self.inventory.iter()
            .map(|item| item.weight())
            .sum::<u32>()
    }
}

pub enum EntityType{
    StillLife,
    Mold,
}

pub struct Entity {
    pub id: u32,
    pub entity_type: EntityType,
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