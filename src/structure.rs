use std::collections::HashMap;

type NodeId = u64;
type EdgeId = u64;

// The unique properties of the edge based on WHICH side you enter from.
struct TraversalData {
    destination: NodeId,
    discover_difficulty: u32, // Ticks required to notice the door
    transit_difficulty: u32,  // Stamina/Resources required to cross
}

struct Edge {
    id: EdgeId,
    length: f32, // The base time it takes to walk through the threshold
    
    // Key: The node the explorer is currently standing in.
    // Value: The destination and the specific difficulties from this side.
    routing_table: HashMap<NodeId, TraversalData>, 
}

struct Node {
    id: NodeId,
    size: u32,                  // e.g., 100 sq ft vs 5000 sq ft
    max_device_capacity: u32,   // Derived from size (how much corporate hardware fits)
    devices: Vec<Item>,         // The placed hardware
    connected_edges: Vec<EdgeId>, 
    
    // The engine tracks how much time explorers have spent searching this specific room.
    discovery_progress: u32,
    pub active_entity: Option<Entity>,
}

// 3. The Corporate Master Map
struct AnomalyGraph {
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<EdgeId, Edge>,
}