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