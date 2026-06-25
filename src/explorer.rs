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