/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! RelationshipSpace - ConceptualSpace specialized for relationships
//!
//! A conceptual space that contains relationship concepts (edges and hyperedges)
//! and provides Voronoi tessellation for similarity clustering.

use crate::aggregates::{EdgeConcept, HyperEdgeConcept};
use crate::quality::QualityPoint;
use crate::value_objects::RelationshipId;
use chrono::{DateTime, Utc};
use cim_domain_spaces::{ConceptualSpaceId, TopologicalSpaceId, VoronoiTessellation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// RelationshipSpace - A conceptual space for relationships
///
/// Contains:
/// - Edges (binary relationships)
/// - HyperEdges (N-ary relationships)
/// - Voronoi tessellation for clustering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipSpace {
    /// Unique identifier
    pub id: ConceptualSpaceId,
    /// Name of this relationship space
    pub name: String,
    /// Associated topological space
    pub topology_id: TopologicalSpaceId,

    /// Edge concepts in this space
    pub edges: HashMap<RelationshipId, EdgeConcept>,
    /// HyperEdge concepts in this space
    pub hyperedges: HashMap<RelationshipId, HyperEdgeConcept>,

    /// Voronoi tessellation (computed from relationship positions)
    pub tessellation: Option<VoronoiTessellation>,

    /// Version
    pub version: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl RelationshipSpace {
    /// Create a new relationship space
    pub fn new(name: impl Into<String>, topology_id: TopologicalSpaceId) -> Self {
        let now = Utc::now();
        Self {
            id: ConceptualSpaceId::new(),
            name: name.into(),
            topology_id,
            edges: HashMap::new(),
            hyperedges: HashMap::new(),
            tessellation: None,
            version: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Add an edge to the space
    pub fn add_edge(&mut self, edge: EdgeConcept) {
        self.edges.insert(edge.id, edge);
        self.updated_at = Utc::now();
        self.version += 1;
        // Invalidate tessellation
        self.tessellation = None;
    }

    /// Add a hyperedge to the space
    pub fn add_hyperedge(&mut self, hyperedge: HyperEdgeConcept) {
        self.hyperedges.insert(hyperedge.id, hyperedge);
        self.updated_at = Utc::now();
        self.version += 1;
        // Invalidate tessellation
        self.tessellation = None;
    }

    /// Get an edge by ID
    pub fn get_edge(&self, id: &RelationshipId) -> Option<&EdgeConcept> {
        self.edges.get(id)
    }

    /// Get a hyperedge by ID
    pub fn get_hyperedge(&self, id: &RelationshipId) -> Option<&HyperEdgeConcept> {
        self.hyperedges.get(id)
    }

    /// Get total relationship count
    pub fn relationship_count(&self) -> usize {
        self.edges.len() + self.hyperedges.len()
    }

    /// Find similar edges to a given point in quality space
    pub fn find_similar_edges(&self, point: &QualityPoint, max_distance: f64) -> Vec<&EdgeConcept> {
        self.edges
            .values()
            .filter(|edge| edge.quality_point().distance(point) <= max_distance)
            .collect()
    }

    /// Get all active edges
    pub fn active_edges(&self) -> Vec<&EdgeConcept> {
        self.edges.values().filter(|e| e.is_active()).collect()
    }

    /// Get all active hyperedges
    pub fn active_hyperedges(&self) -> Vec<&HyperEdgeConcept> {
        self.hyperedges.values().filter(|h| h.is_active()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value_objects::{EntityRef, RelationshipCategory};
    use uuid::Uuid;

    #[test]
    fn test_relationship_space_creation() {
        let topo_id = TopologicalSpaceId::new();
        let space = RelationshipSpace::new("Employment Relationships", topo_id);

        assert_eq!(space.name, "Employment Relationships");
        assert_eq!(space.relationship_count(), 0);
    }

    #[test]
    fn test_add_edge_to_space() {
        let topo_id = TopologicalSpaceId::new();
        let mut space = RelationshipSpace::new("Test Space", topo_id);

        let edge = EdgeConcept::new(
            "Test Employment",
            EntityRef::person(Uuid::now_v7()),
            EntityRef::organization(Uuid::now_v7()),
            RelationshipCategory::Employment,
        );

        space.add_edge(edge);
        assert_eq!(space.relationship_count(), 1);
    }
}
