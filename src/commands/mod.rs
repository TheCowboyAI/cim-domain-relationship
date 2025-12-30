/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! Commands for the Relationship Domain
//!
//! Commands express intent to change the state of relationships.
//! They are validated before execution and produce events.

use crate::quality::RelationshipQuality;
use crate::value_objects::{EntityRef, IncidenceMatrix, ParticipantRole, RelationshipCategory, RelationshipId};
use cim_domain::MessageIdentity;
use serde::{Deserialize, Serialize};

// ============================================================================
// Edge Commands
// ============================================================================

/// Commands for EdgeConcept aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeCommand {
    CreateEdge(CreateEdge),
    ActivateEdge(ActivateEdge),
    SuspendEdge(SuspendEdge),
    ResumeEdge(ResumeEdge),
    TerminateEdge(TerminateEdge),
    RejectEdge(RejectEdge),
    UpdateEdgeQuality(UpdateEdgeQuality),
    AddEdgeEvidence(AddEdgeEvidence),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEdge {
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub source: EntityRef,
    pub target: EntityRef,
    pub category: RelationshipCategory,
    pub name: String,
    pub quality: Option<RelationshipQuality>,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateEdge {
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub activated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspendEdge {
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub reason: Option<String>,
    pub suspended_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumeEdge {
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub resumed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateEdge {
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub reason: String,
    pub terminated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RejectEdge {
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub reason: Option<String>,
    pub rejected_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEdgeQuality {
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub new_quality: RelationshipQuality,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddEdgeEvidence {
    pub identity: MessageIdentity,
    pub edge_id: RelationshipId,
    pub evidence_cid: String,
    pub evidence_type: String,
}

// ============================================================================
// HyperEdge Commands
// ============================================================================

/// Commands for HyperEdgeConcept aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HyperEdgeCommand {
    CreateHyperEdge(CreateHyperEdge),
    ActivateHyperEdge(ActivateHyperEdge),
    AddParticipant(AddParticipant),
    RemoveParticipant(RemoveParticipant),
    ChangeParticipantRole(ChangeParticipantRole),
    TerminateHyperEdge(TerminateHyperEdge),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHyperEdge {
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub name: String,
    pub category: RelationshipCategory,
    pub initial_participants: IncidenceMatrix,
    pub created_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateHyperEdge {
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub activated_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddParticipant {
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub participant: EntityRef,
    pub role: ParticipantRole,
    pub weight: f64,
    pub added_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveParticipant {
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub participant: EntityRef,
    pub reason: String,
    pub removed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeParticipantRole {
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub participant: EntityRef,
    pub new_role: ParticipantRole,
    pub changed_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminateHyperEdge {
    pub identity: MessageIdentity,
    pub hyperedge_id: RelationshipId,
    pub reason: String,
    pub terminated_by: String,
}

// ============================================================================
// Unified Relationship Command
// ============================================================================

/// Unified command type for the relationship domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipCommand {
    Edge(EdgeCommand),
    HyperEdge(HyperEdgeCommand),
}

impl From<EdgeCommand> for RelationshipCommand {
    fn from(cmd: EdgeCommand) -> Self {
        RelationshipCommand::Edge(cmd)
    }
}

impl From<HyperEdgeCommand> for RelationshipCommand {
    fn from(cmd: HyperEdgeCommand) -> Self {
        RelationshipCommand::HyperEdge(cmd)
    }
}
