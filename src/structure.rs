use std::collections::HashMap;
use crate::objects::Item;
use crate::objects::Entity;

type NodeId = u64;
type EdgeId = u64;

// The unique properties of the edge based on WHICH side you enter from.
pub struct TraversalData {
    pub destination: NodeId,
    pub discover_difficulty: u32, // Ticks required to notice the door
    pub transit_difficulty: u32,  // Stamina/Resources required to cross
}

pub struct Edge {
    pub id: EdgeId,
    pub length: f32, // The base time it takes to walk through the threshold
    pub routing_table: HashMap<NodeId, TraversalData>, // Key: The node the explorer is currently standing in. Value: The destination and the specific difficulties from this side.
}

pub struct Node {
    pub id: NodeId,
    pub size: u32,                  // e.g., 100 sq ft vs 5000 sq ft
    pub devices: Vec<Item>,         // The placed hardware
    pub connected_edges: Vec<EdgeId>,
    pub layer: u32,
    
    // The engine tracks how much time explorers have spent searching this specific room.
    pub discovery_progress: u32,
    pub active_entity: Option<Entity>,
}

// 3. The Corporate Master Map
pub struct AnomalyGraph {
    pub nodes: HashMap<NodeId, Node>,
    pub edges: HashMap<EdgeId, Edge>,
}

