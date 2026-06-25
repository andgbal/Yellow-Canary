use std::collections::HashMap;
use crate::structure::{AnomalyGraph, Node, Edge, TraversalData};
use std::{collections::VecDeque, u32};

pub struct DifficultyProfile {
    pub max_edges: u32,
    pub spawn_rate_bounds: (f32, f32),
    pub aggression_bounds: (u32, u32),
    pub speed_bounds: (u32, u32),
    pub discover_bounds: (u32, u32),
    pub transit_bounds: (u32, u32),
    pub loop_chance: f32,
    pub edge_length_bounds: (u32, u32),
}

pub struct StructureGenerator{}

impl StructureGenerator {
    /// Evaluates which difficulty configuration applies based on the shortest path edge count to Base.
    pub fn profile_for_layer(&self, layer: u32) -> DifficultyProfile {
        match layer {
            0 => DifficultyProfile {
                max_edges: 1,
                spawn_rate_bounds: (0.0, 0.0),
                aggression_bounds: (0, 0),
                speed_bounds: (0, 0),
                discover_bounds: (0, 0),
                transit_bounds: (0, 0),
                edge_length_bounds: (0, 2),
                loop_chance: 0.0,
            },
            1..=5 => DifficultyProfile {
                max_edges: 4,
                spawn_rate_bounds: (0.01, 0.05),
                aggression_bounds: (10, 30),
                speed_bounds: (30, 50),
                discover_bounds: (0, 10),
                transit_bounds: (0, 2),
                edge_length_bounds: (0, 6),
                loop_chance: 0.0,
            },
            6..=13 => DifficultyProfile {
                max_edges: 6,
                spawn_rate_bounds: (0.10, 0.20),
                aggression_bounds: (40, 70),
                speed_bounds: (50, 80),
                discover_bounds: (20, 50),
                transit_bounds: (5, 15),
                edge_length_bounds: (0, 9),
                loop_chance: 0.0,
            },
            _ => DifficultyProfile {
                max_edges: u32::MAX,
                spawn_rate_bounds: (0.30, 0.60),
                aggression_bounds: (80, 100),
                speed_bounds: (90, 150),
                discover_bounds: (100, 500),
                transit_bounds: (50, 200),
                edge_length_bounds: (0, 10),
                loop_chance: 0.65,
            },
        }
    }
    /// Generates the safe, Euclidean spanning tree layer by layer.
pub fn generate_phase_1_spanning_tree<R: rand::Rng>(&self, graph: &mut AnomalyGraph, target_layers: u32, rng: &mut R) {
        let mut frontier: VecDeque<u64> = VecDeque::new();
        
        let mut next_node_id = 1;
        let mut next_edge_id = 1;

        // 1. Create Base Node (Layer 0) - Fixed safe baseline parameters
        let base_node = Node {
            id: 0,
            layer: 0,
            size: 5000, // Fixed grand staging hub area size
            devices: Vec::new(),
            connected_edges: Vec::new(),
            active_entity: None,
            discovery_progress: 100,
        };
        
        graph.nodes.insert(0, base_node);
        frontier.push_back(0);

        // 2. The Generation Loop
        while let Some(current_id) = frontier.pop_front() {
            let current_layer = graph.nodes.get(&current_id).unwrap().layer;
            
            if current_layer >= target_layers {
                continue; 
            }

            let profile = self.profile_for_layer(current_layer);
            
            // Randomize child node count based on the profile caps
            let generated_children_count = if profile.max_edges <= 1 {
                1
            } else {
                rng.gen_range(1..=profile.max_edges)
            };

            for _ in 0..generated_children_count {
                let child_id = next_node_id;
                next_node_id += 1;
                let child_layer = current_layer + 1;
                let child_profile = self.profile_for_layer(child_layer);

                // Determine entity presence via layer spawn thresholds
                let spawn_roll = rng.gen::<f32>();
                let active_entity = if spawn_roll >= child_profile.spawn_rate_bounds.0 && spawn_roll <= child_profile.spawn_rate_bounds.1 {
                    let e_type = if child_layer <= 5 { EntityType::StillLife } else { EntityType::Mold };
                    Some(Entity {
                        id: child_id,
                        entity_type: e_type,
                        aggression: rng.gen_range(child_profile.aggression_bounds.0..=child_profile.aggression_bounds.1),
                        speed: rng.gen_range(child_profile.speed_bounds.0..=child_profile.speed_bounds.1),
                    })
                } else {
                    None
                };

                // Create the randomized child node matching structure.rs criteria
                let child_node = Node {
                    id: child_id,
                    layer: child_layer,
                    size: rng.gen_range(100..=3000), // Variable space square footage
                    devices: Vec::new(),
                    connected_edges: Vec::new(),
                    discovery_progress: 0, // Unexplored node tracking setup
                    active_entity,
                };

                graph.nodes.insert(child_id, child_node);
                frontier.push_back(child_id);

                // Generate symmetric edge with randomized transit costs
                let edge = self.create_symmetric_edge(next_edge_id, current_id, child_id, &child_profile, rng);
                graph.edges.insert(next_edge_id, edge);
                
                graph.nodes.get_mut(&current_id).unwrap().connected_edges.push(next_edge_id);
                graph.nodes.get_mut(&child_id).unwrap().connected_edges.push(next_edge_id);

                next_edge_id += 1;
            }
        }
    }

    /// Helper to create an A <-> B edge with randomized parameters from bounds
    pub fn create_symmetric_edge<R: rand::Rng>(
        &self, 
        edge_id: u64, 
        node_a: u64, 
        node_b: u64, 
        profile: &DifficultyProfile,
        rng: &mut R,
    ) -> Edge {
        let mut table = HashMap::new();
        
        let discover_difficulty = if profile.discover_bounds.0 == profile.discover_bounds.1 {
            profile.discover_bounds.0
        } else {
            rng.gen_range(profile.discover_bounds.0..=profile.discover_bounds.1)
        };

        let transit_difficulty = if profile.transit_bounds.0 == profile.transit_bounds.1 {
            profile.transit_bounds.0
        } else {
            rng.gen_range(profile.transit_bounds.0..=profile.transit_bounds.1)
        };

        // Path A -> B
        table.insert(node_a, TraversalData { 
            destination: node_b, 
            discover_difficulty, 
            transit_difficulty,
        });
        
        // Path B -> A (Identical Baseline Return)
        table.insert(node_b, TraversalData { 
            destination: node_a, 
            discover_difficulty, 
            transit_difficulty,
        });

        Edge {
            id: edge_id,
            length: 10.0,
            routing_table: table,
        }
    }
}