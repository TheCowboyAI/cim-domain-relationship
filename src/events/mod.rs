/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! Events for the Relationship Domain
//!
//! Immutable facts about what happened in the relationship domain.
//! All state changes are represented as events for event sourcing.

use crate::quality::RelationshipQuality;
use crate::value_objects::{EntityRef, IncidenceMatrix, ParticipantRole, RelationshipCategory, RelationshipId};
use chrono::{DateTime, Utc};
use cim_domain::MessageIdentity;
use cim_domain_spaces::{ConceptId, KnowledgeLevel};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Edge Events
// ============================================================================

/// Events for EdgeConcept aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeEvent {
    EdgeCreated(EdgeCreated),
    EdgeActivated(EdgeActivated),
    EdgeSuspended(EdgeSuspended),
    EdgeTerminated(EdgeTerminated),
    EdgeRejected(EdgeRejected),
    QualityUpdated(EdgeQualityUpdated),
    EvidenceAdded(EdgeEvidenceAdded),
    KnowledgeProgressed(EdgeKnowledgeProgressed),
    PropertyUpdated(EdgePropertyUpdated),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeCreated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub concept_id: ConceptId,
    pub source: EntityRef,
    pub target: EntityRef,
    pub category: RelationshipCategory,
    pub name: String,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeActivated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub activated_by: String,
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeSuspended {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub reason: Option<String>,
    pub suspended_by: String,
    pub suspended_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeTerminated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub reason: String,
    pub terminated_by: String,
    pub terminated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeRejected {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub reason: Option<String>,
    pub rejected_by: String,
    pub rejected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeQualityUpdated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub old_quality: RelationshipQuality,
    pub new_quality: RelationshipQuality,
    pub reason: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeEvidenceAdded {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub evidence_cid: String,
    pub evidence_type: String,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeKnowledgeProgressed {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub from_level: KnowledgeLevel,
    pub to_level: KnowledgeLevel,
    pub new_confidence: f64,
    pub reason: String,
    pub progressed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgePropertyUpdated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub key: String,
    pub value: serde_json::Value,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// HyperEdge Events
// ============================================================================

/// Events for HyperEdgeConcept aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HyperEdgeEvent {
    HyperEdgeCreated(HyperEdgeCreated),
    HyperEdgeActivated(HyperEdgeActivated),
    ParticipantAdded(ParticipantAdded),
    ParticipantRemoved(ParticipantRemoved),
    ParticipantRoleChanged(ParticipantRoleChanged),
    HyperEdgeTerminated(HyperEdgeTerminated),
    HyperEdgeQualityUpdated(HyperEdgeQualityUpdated),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperEdgeCreated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub concept_id: ConceptId,
    pub name: String,
    pub category: RelationshipCategory,
    pub initial_participants: IncidenceMatrix,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperEdgeActivated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub activated_by: String,
    pub activated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantAdded {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub participant: EntityRef,
    pub role: ParticipantRole,
    pub weight: f64,
    pub added_by: String,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantRemoved {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub participant: EntityRef,
    pub reason: String,
    pub removed_by: String,
    pub removed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantRoleChanged {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub participant: EntityRef,
    pub old_role: ParticipantRole,
    pub new_role: ParticipantRole,
    pub changed_by: String,
    pub changed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperEdgeTerminated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub reason: String,
    pub terminated_by: String,
    pub terminated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperEdgeQualityUpdated {
    pub event_id: Uuid,
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub old_quality: RelationshipQuality,
    pub new_quality: RelationshipQuality,
    pub reason: String,
    pub updated_at: DateTime<Utc>,
}

// ============================================================================
// Unified Relationship Event
// ============================================================================

/// Unified event type for the relationship domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipEvent {
    Edge(EdgeEvent),
    HyperEdge(HyperEdgeEvent),
}

impl From<EdgeEvent> for RelationshipEvent {
    fn from(event: EdgeEvent) -> Self {
        RelationshipEvent::Edge(event)
    }
}

impl From<HyperEdgeEvent> for RelationshipEvent {
    fn from(event: HyperEdgeEvent) -> Self {
        RelationshipEvent::HyperEdge(event)
    }
}
