/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! HyperEdgeConcept Aggregate
//!
//! An N-ary relationship among multiple entities, modeled as a Concept
//! in Gärdenfors' Conceptual Space with Quality Dimensions.
//!
//! ## Hyperedge vs Edge
//!
//! - **Edge**: Connects exactly 2 entities (binary relationship)
//! - **HyperEdge**: Connects N entities (N >= 2), enabling complex compositions
//!
//! ## Example Use Cases
//!
//! - Team membership: [Person1, Person2, Person3] -> Team
//! - Project assignment: [Team1, Team2] -> [Project1, Project2]
//! - Document collaboration: [Author1, Author2, Reviewer1] -> Document

use crate::events::HyperEdgeEvent;
use crate::quality::{QualityPoint, RelationshipQuality};
use crate::value_objects::{EntityRef, IncidenceMatrix, ParticipantRole, RelationshipCategory, RelationshipId, ValidityPeriod};
use crate::RelationshipResult;
use chrono::{DateTime, Utc};
use cim_domain::state_machine::State;
use cim_domain_spaces::{ConceptId, KnowledgeLevel, Point3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// HyperEdge State Machine
// ============================================================================

/// State of a hyperedge in its lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HyperEdgeState {
    /// HyperEdge has been created but not yet active
    Forming,
    /// HyperEdge is currently active
    Active,
    /// HyperEdge is being modified (participants changing)
    Restructuring,
    /// HyperEdge has been dissolved
    Dissolved,
}

impl State for HyperEdgeState {
    fn name(&self) -> &'static str {
        match self {
            HyperEdgeState::Forming => "Forming",
            HyperEdgeState::Active => "Active",
            HyperEdgeState::Restructuring => "Restructuring",
            HyperEdgeState::Dissolved => "Dissolved",
        }
    }

    fn is_terminal(&self) -> bool {
        matches!(self, HyperEdgeState::Dissolved)
    }
}

impl HyperEdgeState {
    /// Check if transition from current state to new state is valid
    pub fn can_transition_to(&self, to: &HyperEdgeState) -> bool {
        use HyperEdgeState::*;
        matches!(
            (self, to),
            // From Forming
            (Forming, Active) |
            (Forming, Dissolved) |
            // From Active
            (Active, Restructuring) |
            (Active, Dissolved) |
            // From Restructuring
            (Restructuring, Active) |
            (Restructuring, Dissolved)
        )
    }
}

impl Default for HyperEdgeState {
    fn default() -> Self {
        HyperEdgeState::Forming
    }
}

// ============================================================================
// HyperEdgeConcept Aggregate
// ============================================================================

/// HyperEdgeConcept - N-ary relationship as a Concept in quality space
///
/// Extends the Concept model from cim-domain-spaces with:
/// - Incidence matrix for participant membership
/// - Role assignments per participant
/// - Quality dimensions for conceptual space positioning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperEdgeConcept {
    // ---- Identity ----
    /// Unique identifier for this hyperedge
    pub id: RelationshipId,
    /// Concept ID in the conceptual space
    pub concept_id: ConceptId,

    // ---- Classification ----
    /// Category of this relationship
    pub category: RelationshipCategory,
    /// Human-readable name
    pub name: String,
    /// Description of the relationship
    pub description: Option<String>,

    // ---- Participants ----
    /// Incidence matrix mapping entities to their participation
    pub participants: IncidenceMatrix,

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
    pub state: HyperEdgeState,
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

impl HyperEdgeConcept {
    /// Create a new hyperedge concept
    pub fn new(
        name: impl Into<String>,
        category: RelationshipCategory,
    ) -> Self {
        let now = Utc::now();
        let quality = RelationshipQuality::default();
        let position = quality.to_quality_point().to_point3();

        Self {
            id: RelationshipId::new(),
            concept_id: ConceptId::new(),
            category,
            name: name.into(),
            description: None,
            participants: IncidenceMatrix::new(),
            quality,
            position,
            knowledge_level: KnowledgeLevel::Unknown,
            confidence: 0.0,
            evidence_cids: Vec::new(),
            state: HyperEdgeState::Forming,
            validity: ValidityPeriod::ongoing_now(),
            properties: HashMap::new(),
            version: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Add a participant
    pub fn add_participant(
        &mut self,
        entity_ref: EntityRef,
        role: ParticipantRole,
        weight: f64,
    ) -> Result<(), String> {
        if self.state.is_terminal() {
            return Err("Cannot modify dissolved hyperedge".to_string());
        }
        self.participants.add_participant(entity_ref, role, weight);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Remove a participant
    pub fn remove_participant(&mut self, entity_ref: &EntityRef) -> Result<(), String> {
        if self.state.is_terminal() {
            return Err("Cannot modify dissolved hyperedge".to_string());
        }
        if self.participants.participant_count() <= 2 {
            return Err("HyperEdge must have at least 2 participants".to_string());
        }
        self.participants.remove_participant(entity_ref);
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Get participant count
    pub fn participant_count(&self) -> usize {
        self.participants.participant_count()
    }

    /// Check if hyperedge is currently active
    pub fn is_active(&self) -> bool {
        self.state == HyperEdgeState::Active && self.validity.is_active()
    }

    /// Activate the hyperedge
    pub fn activate(&mut self) -> Result<(), String> {
        if self.participants.participant_count() < 2 {
            return Err("HyperEdge requires at least 2 participants".to_string());
        }
        if !self.state.can_transition_to(&HyperEdgeState::Active) {
            return Err(format!("Cannot activate from {:?} state", self.state));
        }
        self.state = HyperEdgeState::Active;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// Dissolve the hyperedge
    pub fn dissolve(&mut self, reason: impl Into<String>) -> Result<(), String> {
        if !self.state.can_transition_to(&HyperEdgeState::Dissolved) {
            return Err(format!("Cannot dissolve from {:?} state", self.state));
        }
        let now = Utc::now();
        self.validity = self.validity.clone().end(now, reason);
        self.state = HyperEdgeState::Dissolved;
        self.updated_at = now;
        Ok(())
    }

    /// Get the quality point in conceptual space
    pub fn quality_point(&self) -> QualityPoint {
        self.quality.to_quality_point()
    }

    /// Apply an event to produce the next state (pure functional)
    pub fn apply_event_pure(&self, event: &HyperEdgeEvent) -> RelationshipResult<Self> {
        let mut next = self.clone();
        next.version += 1;
        next.updated_at = Utc::now();

        match event {
            HyperEdgeEvent::HyperEdgeCreated(e) => {
                next.id = e.hyperedge_id;
                next.concept_id = e.concept_id;
                next.name = e.name.clone();
                next.category = e.category.clone();
                next.participants = e.initial_participants.clone();
                next.state = HyperEdgeState::Forming;
                next.created_at = e.created_at;
            }

            HyperEdgeEvent::HyperEdgeActivated(_) => {
                next.state = HyperEdgeState::Active;
            }

            HyperEdgeEvent::ParticipantAdded(e) => {
                next.participants.add_participant(
                    e.participant.clone(),
                    e.role.clone(),
                    e.weight,
                );
            }

            HyperEdgeEvent::ParticipantRemoved(e) => {
                next.participants.remove_participant(&e.participant);
            }

            HyperEdgeEvent::ParticipantRoleChanged(e) => {
                // Remove and re-add with new role
                if let Some(entry) = next.participants.remove_participant(&e.participant) {
                    next.participants.add_participant(
                        e.participant.clone(),
                        e.new_role.clone(),
                        entry.weight,
                    );
                }
            }

            HyperEdgeEvent::HyperEdgeTerminated(e) => {
                next.state = HyperEdgeState::Dissolved;
                next.validity = next.validity.clone().end(e.terminated_at, &e.reason);
            }

            HyperEdgeEvent::HyperEdgeQualityUpdated(e) => {
                next.quality = e.new_quality.clone();
                next.position = next.quality.to_quality_point().to_point3();
            }
        }

        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_hyperedge_creation() {
        let hyperedge = HyperEdgeConcept::new(
            "Team Alpha",
            RelationshipCategory::Membership,
        );

        assert_eq!(hyperedge.state, HyperEdgeState::Forming);
        assert_eq!(hyperedge.participant_count(), 0);
    }

    #[test]
    fn test_hyperedge_participants() {
        let mut hyperedge = HyperEdgeConcept::new(
            "Project Team",
            RelationshipCategory::Membership,
        );

        let person1 = EntityRef::person(Uuid::now_v7());
        let person2 = EntityRef::person(Uuid::now_v7());
        let project = EntityRef::organization(Uuid::now_v7());

        hyperedge.add_participant(person1, ParticipantRole::Member, 1.0).unwrap();
        hyperedge.add_participant(person2, ParticipantRole::Leader, 1.0).unwrap();
        hyperedge.add_participant(project, ParticipantRole::Primary, 1.0).unwrap();

        assert_eq!(hyperedge.participant_count(), 3);
    }

    #[test]
    fn test_hyperedge_activation_requires_participants() {
        let mut hyperedge = HyperEdgeConcept::new(
            "Empty Team",
            RelationshipCategory::Membership,
        );

        // Cannot activate without participants
        assert!(hyperedge.activate().is_err());

        // Add participants
        hyperedge.add_participant(EntityRef::person(Uuid::now_v7()), ParticipantRole::Member, 1.0).unwrap();
        hyperedge.add_participant(EntityRef::person(Uuid::now_v7()), ParticipantRole::Member, 1.0).unwrap();

        // Now can activate
        assert!(hyperedge.activate().is_ok());
        assert_eq!(hyperedge.state, HyperEdgeState::Active);
    }

    #[test]
    fn test_hyperedge_minimum_participants() {
        let mut hyperedge = HyperEdgeConcept::new(
            "Duo",
            RelationshipCategory::Membership,
        );

        let person1 = EntityRef::person(Uuid::now_v7());
        let person2 = EntityRef::person(Uuid::now_v7());

        hyperedge.add_participant(person1.clone(), ParticipantRole::Member, 1.0).unwrap();
        hyperedge.add_participant(person2.clone(), ParticipantRole::Member, 1.0).unwrap();
        hyperedge.activate().unwrap();

        // Cannot remove when only 2 participants remain
        assert!(hyperedge.remove_participant(&person1).is_err());
    }
}
