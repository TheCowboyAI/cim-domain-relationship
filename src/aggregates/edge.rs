/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! EdgeConcept Aggregate
//!
//! A binary relationship between two entities, modeled as a Concept in
//! Gärdenfors' Conceptual Space with Quality Dimensions.
//!
//! ## Design Principles
//!
//! - **Is a Concept**: EdgeConcept extends the Concept type from cim-domain-spaces
//! - **Has Quality Dimensions**: Position in 5D quality space (strength, trust, etc.)
//! - **Has Entity References**: CID-addressed source and target entities
//! - **Has State Machine**: Mealy machine for lifecycle transitions
//! - **Event Sourced**: All changes via immutable events

use crate::events::EdgeEvent;
use crate::quality::{QualityPoint, RelationshipQuality};
use crate::value_objects::{EntityRef, RelationshipCategory, RelationshipId, ValidityPeriod};
use crate::RelationshipResult;
use chrono::{DateTime, Utc};
use cim_domain::state_machine::State;
use cim_domain_spaces::{ConceptId, KnowledgeLevel, Point3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Edge State Machine
// ============================================================================

/// State of an edge in its lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeState {
    /// Edge has been created but not yet active
    Proposed,
    /// Edge is currently active
    Active,
    /// Edge has been suspended (temporarily inactive)
    Suspended,
    /// Edge has been terminated
    Terminated,
    /// Edge was rejected (never became active)
    Rejected,
}

impl State for EdgeState {
    fn name(&self) -> &'static str {
        match self {
            EdgeState::Proposed => "Proposed",
            EdgeState::Active => "Active",
            EdgeState::Suspended => "Suspended",
            EdgeState::Terminated => "Terminated",
            EdgeState::Rejected => "Rejected",
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self, EdgeState::Terminated | EdgeState::Rejected)
    }
}

impl EdgeState {
    /// Check if transition from current state to new state is valid
    pub fn can_transition_to(&self, to: &EdgeState) -> bool {
        use EdgeState::*;
        matches!(
            (self, to),
            // From Proposed
            (Proposed, Active) |
            (Proposed, Rejected) |
            // From Active
            (Active, Suspended) |
            (Active, Terminated) |
            // From Suspended
            (Suspended, Active) |
            (Suspended, Terminated)
        )
    }

    /// Get valid next states from current state
    pub fn valid_transitions(&self) -> Vec<EdgeState> {
        use EdgeState::*;
        match self {
            Proposed => vec![Active, Rejected],
            Active => vec![Suspended, Terminated],
            Suspended => vec![Active, Terminated],
            Terminated => vec![],
            Rejected => vec![],
        }
    }
}

impl Default for EdgeState {
    fn default() -> Self {
        EdgeState::Proposed
    }
}

// ============================================================================
// EdgeConcept Aggregate
// ============================================================================

/// EdgeConcept - Binary relationship as a Concept in quality space
///
/// Extends the Concept model from cim-domain-spaces with:
/// - Source and target entity references
/// - Relationship category
/// - Quality dimensions for conceptual space positioning
///
/// ## Example
///
/// ```rust,ignore
/// // Create an employment relationship
/// let employment = EdgeConcept::new(
///     "Steele's Employment at Cowboy AI",
///     EntityRef::person(steele_id),
///     EntityRef::organization(cowboy_ai_id),
///     RelationshipCategory::Employment,
/// ).with_quality(RelationshipQuality::default_employment());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeConcept {
    // ---- Identity ----
    /// Unique identifier for this edge
    pub id: RelationshipId,
    /// Concept ID in the conceptual space
    pub concept_id: ConceptId,

    // ---- Relationship Endpoints ----
    /// Source entity (the "from" end of the edge)
    pub source: EntityRef,
    /// Target entity (the "to" end of the edge)
    pub target: EntityRef,

    // ---- Classification ----
    /// Category of this relationship
    pub category: RelationshipCategory,
    /// Human-readable name
    pub name: String,
    /// Description of the relationship
    pub description: Option<String>,

    // ---- Quality Space Position ----
    /// Quality dimensions as a point in conceptual space
    pub quality: RelationshipQuality,
    /// Position in 3D for visualization (derived from quality)
    pub position: Point3<f64>,

    // ---- Knowledge State ----
    /// How well-known this relationship is
    pub knowledge_level: KnowledgeLevel,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Evidence CIDs supporting this relationship
    pub evidence_cids: Vec<String>,

    // ---- Lifecycle ----
    /// Current state in the lifecycle
    pub state: EdgeState,
    /// Validity period
    pub validity: ValidityPeriod,

    // ---- Metadata ----
    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Event version
    pub version: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl EdgeConcept {
    /// Create a new edge concept
    pub fn new(
        name: impl Into<String>,
        source: EntityRef,
        target: EntityRef,
        category: RelationshipCategory,
    ) -> Self {
        let now = Utc::now();
        let quality = RelationshipQuality::default();
        let position = quality.to_quality_point().to_point3();

        Self {
            id: RelationshipId::new(),
            concept_id: ConceptId::new(),
            source,
            target,
            category,
            name: name.into(),
            description: None,
            quality,
            position,
            knowledge_level: KnowledgeLevel::Unknown,
            confidence: 0.0,
            evidence_cids: Vec::new(),
            state: EdgeState::Proposed,
            validity: ValidityPeriod::ongoing_now(),
            properties: HashMap::new(),
            version: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set the quality dimensions
    pub fn with_quality(mut self, quality: RelationshipQuality) -> Self {
        self.position = quality.to_quality_point().to_point3();
        self.quality = quality;
        self
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the validity period
    pub fn with_validity(mut self, validity: ValidityPeriod) -> Self {
        self.validity = validity;
        self
    }

    /// Add a property
    pub fn with_property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }

    // ---- State Machine ----

    /// Transition to a new state
    pub fn transition_to(&mut self, new_state: EdgeState) -> Result<(), String> {
        if self.state.can_transition_to(&new_state) {
            self.state = new_state;
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(format!(
                "Cannot transition from {:?} to {:?}",
                self.state, new_state
            ))
        }
    }

    /// Activate the edge (from Proposed state)
    pub fn activate(&mut self) -> Result<(), String> {
        self.transition_to(EdgeState::Active)
    }

    /// Suspend the edge (from Active state)
    pub fn suspend(&mut self) -> Result<(), String> {
        self.transition_to(EdgeState::Suspended)
    }

    /// Resume from suspension (from Suspended state)
    pub fn resume(&mut self) -> Result<(), String> {
        self.transition_to(EdgeState::Active)
    }

    /// Terminate the edge
    pub fn terminate(&mut self, reason: impl Into<String>) -> Result<(), String> {
        let now = Utc::now();
        self.validity = self.validity.clone().end(now, reason);
        self.transition_to(EdgeState::Terminated)
    }

    /// Reject the edge (from Proposed state)
    pub fn reject(&mut self) -> Result<(), String> {
        self.transition_to(EdgeState::Rejected)
    }

    // ---- Query Methods ----

    /// Check if the edge is currently active
    pub fn is_active(&self) -> bool {
        self.state == EdgeState::Active && self.validity.is_active()
    }

    /// Check if this is a symmetric (bidirectional) relationship
    pub fn is_symmetric(&self) -> bool {
        self.category.is_symmetric()
    }

    /// Get the quality point in conceptual space
    pub fn quality_point(&self) -> QualityPoint {
        self.quality.to_quality_point()
    }

    /// Calculate similarity to another edge (based on quality space distance)
    pub fn similarity(&self, other: &EdgeConcept) -> f64 {
        let distance = self.quality_point().distance(&other.quality_point());
        // Convert distance to similarity (0 distance = 1.0 similarity)
        // Max distance in 5D unit cube is sqrt(5) ≈ 2.236
        1.0 - (distance / 2.236).min(1.0)
    }

    // ---- Event Sourcing ----

    /// Apply an event to produce the next state (pure functional)
    pub fn apply_event_pure(&self, event: &EdgeEvent) -> RelationshipResult<Self> {
        let mut next = self.clone();
        next.version += 1;
        next.updated_at = Utc::now();

        match event {
            EdgeEvent::EdgeCreated(e) => {
                next.id = e.edge_id;
                next.concept_id = e.concept_id;
                next.source = e.source.clone();
                next.target = e.target.clone();
                next.category = e.category.clone();
                next.name = e.name.clone();
                next.state = EdgeState::Proposed;
                next.created_at = e.created_at;
            }

            EdgeEvent::EdgeActivated(_) => {
                next.state = EdgeState::Active;
            }

            EdgeEvent::EdgeSuspended(e) => {
                next.state = EdgeState::Suspended;
                if let Some(ref reason) = e.reason {
                    next.properties.insert(
                        "suspension_reason".to_string(),
                        serde_json::Value::String(reason.clone()),
                    );
                }
            }

            EdgeEvent::EdgeTerminated(e) => {
                next.state = EdgeState::Terminated;
                next.validity = next.validity.clone().end(e.terminated_at, &e.reason);
            }

            EdgeEvent::EdgeRejected(e) => {
                next.state = EdgeState::Rejected;
                if let Some(ref reason) = e.reason {
                    next.properties.insert(
                        "rejection_reason".to_string(),
                        serde_json::Value::String(reason.clone()),
                    );
                }
            }

            EdgeEvent::QualityUpdated(e) => {
                next.quality = e.new_quality.clone();
                next.position = next.quality.to_quality_point().to_point3();
            }

            EdgeEvent::EvidenceAdded(e) => {
                if !next.evidence_cids.contains(&e.evidence_cid) {
                    next.evidence_cids.push(e.evidence_cid.clone());
                }
                // Update confidence based on evidence
                next.confidence = (next.evidence_cids.len() as f64 / 10.0).min(1.0);
            }

            EdgeEvent::KnowledgeProgressed(e) => {
                next.knowledge_level = e.to_level;
                next.confidence = e.new_confidence;
            }

            EdgeEvent::PropertyUpdated(e) => {
                next.properties.insert(e.key.clone(), e.value.clone());
            }
        }

        Ok(next)
    }

    /// Rebuild aggregate from event history
    pub fn from_events(events: &[EdgeEvent]) -> RelationshipResult<Self> {
        if events.is_empty() {
            return Err(crate::RelationshipError::InvalidRelationship(
                "No events provided".to_string(),
            ));
        }

        // Start with placeholder that will be overwritten by first event
        let first_event = &events[0];
        let mut edge = match first_event {
            EdgeEvent::EdgeCreated(e) => {
                let quality = RelationshipQuality::default();
                Self {
                    id: e.edge_id,
                    concept_id: e.concept_id,
                    source: e.source.clone(),
                    target: e.target.clone(),
                    category: e.category.clone(),
                    name: e.name.clone(),
                    description: None,
                    quality: quality.clone(),
                    position: quality.to_quality_point().to_point3(),
                    knowledge_level: KnowledgeLevel::Unknown,
                    confidence: 0.0,
                    evidence_cids: Vec::new(),
                    state: EdgeState::Proposed,
                    validity: ValidityPeriod::ongoing(e.created_at),
                    properties: HashMap::new(),
                    version: 0,
                    created_at: e.created_at,
                    updated_at: e.created_at,
                }
            }
            _ => {
                return Err(crate::RelationshipError::InvalidRelationship(
                    "First event must be EdgeCreated".to_string(),
                ))
            }
        };

        // Apply remaining events
        for event in &events[1..] {
            edge = edge.apply_event_pure(event)?;
        }

        Ok(edge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_edge_creation() {
        let source = EntityRef::person(Uuid::now_v7());
        let target = EntityRef::organization(Uuid::now_v7());

        let edge = EdgeConcept::new(
            "Test Employment",
            source.clone(),
            target.clone(),
            RelationshipCategory::Employment,
        );

        assert_eq!(edge.state, EdgeState::Proposed);
        assert_eq!(edge.source, source);
        assert_eq!(edge.target, target);
    }

    #[test]
    fn test_edge_state_transitions() {
        let source = EntityRef::person(Uuid::now_v7());
        let target = EntityRef::organization(Uuid::now_v7());

        let mut edge = EdgeConcept::new(
            "Test",
            source,
            target,
            RelationshipCategory::Employment,
        );

        // Proposed -> Active
        assert!(edge.activate().is_ok());
        assert_eq!(edge.state, EdgeState::Active);

        // Active -> Suspended
        assert!(edge.suspend().is_ok());
        assert_eq!(edge.state, EdgeState::Suspended);

        // Suspended -> Active
        assert!(edge.resume().is_ok());
        assert_eq!(edge.state, EdgeState::Active);

        // Active -> Terminated
        assert!(edge.terminate("End of contract").is_ok());
        assert_eq!(edge.state, EdgeState::Terminated);
    }

    #[test]
    fn test_invalid_transition() {
        let source = EntityRef::person(Uuid::now_v7());
        let target = EntityRef::organization(Uuid::now_v7());

        let mut edge = EdgeConcept::new(
            "Test",
            source,
            target,
            RelationshipCategory::Employment,
        );

        // Cannot go directly from Proposed to Terminated
        assert!(edge.terminate("Invalid").is_err());
    }

    #[test]
    fn test_similarity() {
        let source1 = EntityRef::person(Uuid::now_v7());
        let target1 = EntityRef::organization(Uuid::now_v7());

        let edge1 = EdgeConcept::new("Employment 1", source1.clone(), target1.clone(), RelationshipCategory::Employment)
            .with_quality(RelationshipQuality::default_employment());

        let source2 = EntityRef::person(Uuid::now_v7());
        let target2 = EntityRef::organization(Uuid::now_v7());

        let edge2 = EdgeConcept::new("Employment 2", source2, target2, RelationshipCategory::Employment)
            .with_quality(RelationshipQuality::default_employment());

        // Same quality should have high similarity
        let similarity = edge1.similarity(&edge2);
        assert!(similarity > 0.9);

        // Different quality should have lower similarity
        let edge3 = EdgeConcept::new("Friendship", source1, target1, RelationshipCategory::Friendship)
            .with_quality(RelationshipQuality::default_friendship());

        let similarity = edge1.similarity(&edge3);
        assert!(similarity < 0.7);
    }
}
