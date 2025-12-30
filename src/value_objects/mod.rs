/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! Value Objects for the Relationship Domain
//!
//! Core value types representing relationship concepts:
//! - EntityRef: Content-addressed reference to any domain entity
//! - RelationshipId: Unique identifier for relationships
//! - RelationshipCategory: Classification of relationship types
//! - ValidityPeriod: Temporal bounds for relationships
//! - IncidenceMatrix: Sparse representation for hyperedge membership
//! - ParticipantRole: Role assignment for hyperedge participants

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;
use uuid::Uuid;

// ============================================================================
// ID Types
// ============================================================================

/// Unique identifier for a relationship (Edge or HyperEdge)
///
/// Phantom-typed for compile-time safety:
/// - RelationshipId<Person, Organization> for employment
/// - RelationshipId<Team, Project> for assignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelationshipId<Source = (), Target = ()> {
    id: Uuid,
    #[serde(skip)]
    _source: PhantomData<Source>,
    #[serde(skip)]
    _target: PhantomData<Target>,
}

impl<S, T> RelationshipId<S, T> {
    /// Create a new time-ordered relationship ID
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            _source: PhantomData,
            _target: PhantomData,
        }
    }

    /// Create from existing UUID
    pub fn from_uuid(id: Uuid) -> Self {
        Self {
            id,
            _source: PhantomData,
            _target: PhantomData,
        }
    }

    /// Get the inner UUID
    pub fn as_uuid(&self) -> Uuid {
        self.id
    }

    /// Erase type information for storage
    pub fn erase_types(self) -> RelationshipId<(), ()> {
        RelationshipId {
            id: self.id,
            _source: PhantomData,
            _target: PhantomData,
        }
    }
}

impl<S, T> Default for RelationshipId<S, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, T> std::fmt::Display for RelationshipId<S, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Relationship({})", self.id)
    }
}

// ============================================================================
// Entity Reference (CID-Addressed)
// ============================================================================

/// Type of entity being referenced
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityType {
    /// Person entity from cim-domain-person
    Person,
    /// Organization entity from cim-domain-organization
    Organization,
    /// Location entity from cim-domain-location
    Location,
    /// Agent entity from cim-domain-agent
    Agent,
    /// Policy entity from cim-domain-policy
    Policy,
    /// Concept from any ConceptualSpace
    Concept,
    /// Another relationship (for relationship-to-relationship edges)
    Relationship,
    /// Custom domain entity
    Custom(String),
}

impl EntityType {
    /// Get the NATS subject prefix for this entity type
    pub fn nats_subject_prefix(&self) -> &'static str {
        match self {
            EntityType::Person => "person",
            EntityType::Organization => "organization",
            EntityType::Location => "location",
            EntityType::Agent => "agent",
            EntityType::Policy => "policy",
            EntityType::Concept => "concept",
            EntityType::Relationship => "relationship",
            EntityType::Custom(_) => "custom",
        }
    }
}

/// Content-addressed reference to any domain entity
///
/// EntityRef provides CID-based immutable references that:
/// - Can resolve across domain boundaries
/// - Are content-addressed for integrity
/// - Support version pinning via CID
///
/// ```rust,ignore
/// let person_ref = EntityRef::person(person_id)
///     .with_cid(person_cid);
///
/// let org_ref = EntityRef::organization(org_id)
///     .with_version(3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityRef {
    /// Type of entity being referenced
    pub entity_type: EntityType,

    /// Entity's unique identifier (UUID v7)
    pub entity_id: Uuid,

    /// Optional CID for content-addressed reference
    /// When present, pins to a specific version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,

    /// Optional version number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<u64>,
}

impl EntityRef {
    /// Create a new entity reference
    pub fn new(entity_type: EntityType, entity_id: Uuid) -> Self {
        Self {
            entity_type,
            entity_id,
            cid: None,
            version: None,
        }
    }

    /// Create a reference to a Person
    pub fn person(id: Uuid) -> Self {
        Self::new(EntityType::Person, id)
    }

    /// Create a reference to an Organization
    pub fn organization(id: Uuid) -> Self {
        Self::new(EntityType::Organization, id)
    }

    /// Create a reference to a Location
    pub fn location(id: Uuid) -> Self {
        Self::new(EntityType::Location, id)
    }

    /// Create a reference to an Agent
    pub fn agent(id: Uuid) -> Self {
        Self::new(EntityType::Agent, id)
    }

    /// Create a reference to a Concept
    pub fn concept(id: Uuid) -> Self {
        Self::new(EntityType::Concept, id)
    }

    /// Create a reference to another Relationship
    pub fn relationship(id: Uuid) -> Self {
        Self::new(EntityType::Relationship, id)
    }

    /// Add CID for content-addressed pinning
    pub fn with_cid(mut self, cid: impl Into<String>) -> Self {
        self.cid = Some(cid.into());
        self
    }

    /// Add version number
    pub fn with_version(mut self, version: u64) -> Self {
        self.version = Some(version);
        self
    }

    /// Generate NATS subject for fetching this entity
    pub fn to_nats_subject(&self) -> String {
        format!(
            "{}.query.get.{}",
            self.entity_type.nats_subject_prefix(),
            self.entity_id
        )
    }

    /// Check if this reference is pinned to a specific version
    pub fn is_pinned(&self) -> bool {
        self.cid.is_some() || self.version.is_some()
    }
}

impl std::fmt::Display for EntityRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.entity_type.nats_subject_prefix(), self.entity_id)?;
        if let Some(ref cid) = self.cid {
            write!(f, "@{}", &cid[..8])?; // Abbreviated CID
        } else if let Some(v) = self.version {
            write!(f, "@v{}", v)?;
        }
        Ok(())
    }
}

// ============================================================================
// Relationship Categories
// ============================================================================

/// High-level classification of relationship types
///
/// Categories define the semantic meaning of relationships in the domain.
/// Each category has associated quality dimension defaults.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelationshipCategory {
    // ---- Organizational Relationships ----
    /// Employment relationship (Person -> Organization)
    Employment,
    /// Membership relationship (Person -> Group/Organization)
    Membership,
    /// Ownership relationship (Entity -> Asset)
    Ownership,
    /// Management relationship (Person -> Person/Team)
    Management,

    // ---- Social Relationships ----
    /// Friendship relationship (Person <-> Person)
    Friendship,
    /// Professional contact (Person <-> Person)
    ProfessionalContact,
    /// Mentorship (Person -> Person)
    Mentorship,

    // ---- Structural Relationships ----
    /// Part-of relationship (Component -> Whole)
    PartOf,
    /// Contains relationship (Container -> Contents)
    Contains,
    /// Depends-on relationship (Dependent -> Dependency)
    DependsOn,
    /// Implements relationship (Implementation -> Specification)
    Implements,

    // ---- Temporal Relationships ----
    /// Precedes relationship (Event -> Event)
    Precedes,
    /// Triggers relationship (Cause -> Effect)
    Triggers,

    // ---- Knowledge Relationships ----
    /// References relationship (Document -> Document)
    References,
    /// Derives-from relationship (Derivative -> Source)
    DerivesFrom,

    // ---- Custom Category ----
    /// Domain-specific relationship
    Custom(String),
}

impl RelationshipCategory {
    /// Get the default formality for this category
    pub fn default_formality(&self) -> Formality {
        match self {
            RelationshipCategory::Employment => Formality::Contractual,
            RelationshipCategory::Ownership => Formality::Legal,
            RelationshipCategory::Membership => Formality::Formal,
            RelationshipCategory::Friendship => Formality::Informal,
            RelationshipCategory::ProfessionalContact => Formality::SemiFormal,
            _ => Formality::Formal,
        }
    }

    /// Check if this relationship type is typically bidirectional
    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            RelationshipCategory::Friendship | RelationshipCategory::ProfessionalContact
        )
    }

    /// Get human-readable name
    pub fn display_name(&self) -> String {
        match self {
            RelationshipCategory::Employment => "employment".to_string(),
            RelationshipCategory::Membership => "membership".to_string(),
            RelationshipCategory::Ownership => "ownership".to_string(),
            RelationshipCategory::Management => "management".to_string(),
            RelationshipCategory::Friendship => "friendship".to_string(),
            RelationshipCategory::ProfessionalContact => "professional contact".to_string(),
            RelationshipCategory::Mentorship => "mentorship".to_string(),
            RelationshipCategory::PartOf => "part of".to_string(),
            RelationshipCategory::Contains => "contains".to_string(),
            RelationshipCategory::DependsOn => "depends on".to_string(),
            RelationshipCategory::Implements => "implements".to_string(),
            RelationshipCategory::Precedes => "precedes".to_string(),
            RelationshipCategory::Triggers => "triggers".to_string(),
            RelationshipCategory::References => "references".to_string(),
            RelationshipCategory::DerivesFrom => "derives from".to_string(),
            RelationshipCategory::Custom(name) => name.clone(),
        }
    }
}

// ============================================================================
// Validity Period
// ============================================================================

/// Temporal bounds for a relationship
///
/// Supports:
/// - Open-ended (ongoing) relationships
/// - Fixed-term relationships
/// - Historical (ended) relationships
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidityPeriod {
    /// When the relationship started
    pub starts_at: DateTime<Utc>,

    /// When the relationship ends (None = ongoing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<DateTime<Utc>>,

    /// Why the relationship ended (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<String>,
}

impl ValidityPeriod {
    /// Create an ongoing relationship starting now
    pub fn ongoing_now() -> Self {
        Self {
            starts_at: Utc::now(),
            ends_at: None,
            end_reason: None,
        }
    }

    /// Create an ongoing relationship starting at a specific time
    pub fn ongoing(starts_at: DateTime<Utc>) -> Self {
        Self {
            starts_at,
            ends_at: None,
            end_reason: None,
        }
    }

    /// Create a fixed-term relationship
    pub fn fixed_term(starts_at: DateTime<Utc>, ends_at: DateTime<Utc>) -> Self {
        Self {
            starts_at,
            ends_at: Some(ends_at),
            end_reason: None,
        }
    }

    /// End an ongoing relationship
    pub fn end(mut self, ends_at: DateTime<Utc>, reason: impl Into<String>) -> Self {
        self.ends_at = Some(ends_at);
        self.end_reason = Some(reason.into());
        self
    }

    /// Check if relationship is currently active
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        self.starts_at <= now && self.ends_at.map_or(true, |end| now < end)
    }

    /// Check if relationship has ended
    pub fn has_ended(&self) -> bool {
        self.ends_at.map_or(false, |end| Utc::now() >= end)
    }

    /// Get duration in days (None if ongoing)
    pub fn duration_days(&self) -> Option<i64> {
        self.ends_at.map(|end| (end - self.starts_at).num_days())
    }
}

impl Default for ValidityPeriod {
    fn default() -> Self {
        Self::ongoing_now()
    }
}

// ============================================================================
// Formality Levels
// ============================================================================

/// Level of formality for a relationship
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Formality {
    /// Casual, no documentation
    Informal,
    /// Documented but not binding
    SemiFormal,
    /// Official, documented relationship
    Formal,
    /// Contract-based relationship
    Contractual,
    /// Legally binding relationship
    Legal,
}

impl Formality {
    /// Convert to numeric value for quality space
    pub fn as_f64(&self) -> f64 {
        match self {
            Formality::Informal => 0.0,
            Formality::SemiFormal => 0.25,
            Formality::Formal => 0.5,
            Formality::Contractual => 0.75,
            Formality::Legal => 1.0,
        }
    }

    /// Create from numeric value
    pub fn from_f64(value: f64) -> Self {
        if value < 0.125 {
            Formality::Informal
        } else if value < 0.375 {
            Formality::SemiFormal
        } else if value < 0.625 {
            Formality::Formal
        } else if value < 0.875 {
            Formality::Contractual
        } else {
            Formality::Legal
        }
    }
}

// ============================================================================
// Incidence Matrix (for HyperEdges)
// ============================================================================

/// Sparse incidence matrix for hyperedge membership
///
/// Maps entity references to their participation in the hyperedge.
/// Each participant has a role assignment.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IncidenceMatrix {
    /// Participants and their roles
    participants: HashMap<String, ParticipantEntry>,
}

/// Entry in the incidence matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantEntry {
    /// Reference to the participating entity
    pub entity_ref: EntityRef,
    /// Role in this hyperedge
    pub role: ParticipantRole,
    /// Participation weight (0.0 - 1.0)
    pub weight: f64,
    /// When this participant joined
    pub joined_at: DateTime<Utc>,
}

impl IncidenceMatrix {
    /// Create an empty incidence matrix
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a participant
    pub fn add_participant(
        &mut self,
        entity_ref: EntityRef,
        role: ParticipantRole,
        weight: f64,
    ) {
        let key = entity_ref.to_string();
        self.participants.insert(
            key,
            ParticipantEntry {
                entity_ref,
                role,
                weight: weight.clamp(0.0, 1.0),
                joined_at: Utc::now(),
            },
        );
    }

    /// Remove a participant
    pub fn remove_participant(&mut self, entity_ref: &EntityRef) -> Option<ParticipantEntry> {
        self.participants.remove(&entity_ref.to_string())
    }

    /// Get participant count
    pub fn participant_count(&self) -> usize {
        self.participants.len()
    }

    /// Get all participants
    pub fn participants(&self) -> impl Iterator<Item = &ParticipantEntry> {
        self.participants.values()
    }

    /// Get participants by role
    pub fn participants_with_role(&self, role: &ParticipantRole) -> Vec<&ParticipantEntry> {
        self.participants
            .values()
            .filter(|p| &p.role == role)
            .collect()
    }

    /// Check if entity is a participant
    pub fn contains(&self, entity_ref: &EntityRef) -> bool {
        self.participants.contains_key(&entity_ref.to_string())
    }
}

// ============================================================================
// Participant Roles
// ============================================================================

/// Role of a participant in a hyperedge
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ParticipantRole {
    // ---- Generic Roles ----
    /// Primary entity in the relationship
    Primary,
    /// Secondary entity in the relationship
    Secondary,
    /// Observer of the relationship
    Observer,
    /// Facilitator of the relationship
    Facilitator,

    // ---- Team Roles ----
    /// Leader of a team/group
    Leader,
    /// Member of a team/group
    Member,
    /// Contributor to a project
    Contributor,
    /// Stakeholder with interest
    Stakeholder,

    // ---- Document Roles ----
    /// Author of a document
    Author,
    /// Reviewer of a document
    Reviewer,
    /// Approver of a document
    Approver,

    // ---- Custom Role ----
    /// Domain-specific role
    Custom(String),
}

impl ParticipantRole {
    /// Get human-readable name
    pub fn display_name(&self) -> String {
        match self {
            ParticipantRole::Primary => "primary".to_string(),
            ParticipantRole::Secondary => "secondary".to_string(),
            ParticipantRole::Observer => "observer".to_string(),
            ParticipantRole::Facilitator => "facilitator".to_string(),
            ParticipantRole::Leader => "leader".to_string(),
            ParticipantRole::Member => "member".to_string(),
            ParticipantRole::Contributor => "contributor".to_string(),
            ParticipantRole::Stakeholder => "stakeholder".to_string(),
            ParticipantRole::Author => "author".to_string(),
            ParticipantRole::Reviewer => "reviewer".to_string(),
            ParticipantRole::Approver => "approver".to_string(),
            ParticipantRole::Custom(name) => name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_id_creation() {
        let id1: RelationshipId<(), ()> = RelationshipId::new();
        let id2: RelationshipId<(), ()> = RelationshipId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_entity_ref_display() {
        let person_ref = EntityRef::person(Uuid::now_v7());
        assert!(person_ref.to_string().starts_with("person:"));

        let pinned_ref = EntityRef::organization(Uuid::now_v7())
            .with_version(5);
        assert!(pinned_ref.to_string().contains("@v5"));
    }

    #[test]
    fn test_validity_period() {
        let ongoing = ValidityPeriod::ongoing_now();
        assert!(ongoing.is_active());
        assert!(!ongoing.has_ended());

        let past_start = Utc::now() - chrono::Duration::days(30);
        let past_end = Utc::now() - chrono::Duration::days(10);
        let ended = ValidityPeriod::fixed_term(past_start, past_end);
        assert!(ended.has_ended());
    }

    #[test]
    fn test_formality_conversion() {
        assert_eq!(Formality::from_f64(0.0), Formality::Informal);
        assert_eq!(Formality::from_f64(0.5), Formality::Formal);
        assert_eq!(Formality::from_f64(1.0), Formality::Legal);
    }

    #[test]
    fn test_incidence_matrix() {
        let mut matrix = IncidenceMatrix::new();

        let person = EntityRef::person(Uuid::now_v7());
        let org = EntityRef::organization(Uuid::now_v7());

        matrix.add_participant(person.clone(), ParticipantRole::Primary, 1.0);
        matrix.add_participant(org.clone(), ParticipantRole::Secondary, 0.8);

        assert_eq!(matrix.participant_count(), 2);
        assert!(matrix.contains(&person));
        assert!(matrix.contains(&org));
    }
}
