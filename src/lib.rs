/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! # CIM Domain Relationship
//!
//! Relationship domain for the Composable Information Machine (CIM).
//!
//! ## Core Principle: Relationships are Concepts
//!
//! Following Gärdenfors' Conceptual Spaces theory, relationships are NOT simple
//! triples (subject -- predicate -> object). They are **Concepts** with:
//!
//! - **Quality Dimensions**: Strength, trust, duration, formality, etc.
//! - **Position in Conceptual Space**: Relationships can be compared for similarity
//! - **Voronoi Tessellation**: Relationships cluster naturally in conceptual space
//! - **Knowledge Levels**: Unknown -> Suspected -> Known progression
//!
//! ## Architecture
//!
//! ```text
//! RelationshipSpace (ConceptualSpace)
//!     |
//!     +-- Quality Dimensions
//!     |   +-- Strength (0.0 - 1.0)
//!     |   +-- Trust (0.0 - 1.0)
//!     |   +-- Formality (informal -> legal)
//!     |   +-- Duration (temporal bounds)
//!     |   +-- Reciprocity (unidirectional -> mutual)
//!     |
//!     +-- EdgeConcept (extends Concept)
//!     |   +-- Source EntityRef (CID-addressed)
//!     |   +-- Target EntityRef (CID-addressed)
//!     |   +-- RelationshipCategory
//!     |   +-- Position in Quality Space
//!     |
//!     +-- HyperEdgeConcept (extends Concept)
//!         +-- IncidenceMatrix (participants)
//!         +-- Role assignments per participant
//!         +-- Collective relationship type
//! ```
//!
//! ## Graph Theory Foundation
//!
//! - **Edge**: Binary relationship (Person -> Organization)
//! - **HyperEdge**: N-ary relationship ([Team] -> [Project] -> [Milestone])
//! - **Incidence Matrix**: Sparse representation of entity participation
//!
//! ## Category Theory Compliance
//!
//! - Relationships are morphisms between entity categories
//! - Composition preserves structure (Employment o Membership = Authority)
//! - Functor maps relationships to/from Graph structures
//!
//! ## Usage
//!
//! ```rust,ignore
//! use cim_domain_relationship::{EdgeConcept, HyperEdgeConcept, EntityRef};
//! use cim_domain_relationship::quality::{RelationshipQuality, QualityPoint};
//!
//! // Create an employment relationship as a Concept in quality space
//! let employment = EdgeConcept::new(
//!     EntityRef::person(person_id),
//!     EntityRef::organization(org_id),
//!     RelationshipCategory::Employment,
//! ).with_quality(RelationshipQuality {
//!     strength: 0.8,
//!     trust: 0.9,
//!     formality: Formality::Contractual,
//!     duration: ValidityPeriod::ongoing(start_date),
//!     reciprocity: 0.7,
//! });
//! ```

pub mod aggregates;
pub mod value_objects;
pub mod events;
pub mod commands;
pub mod infrastructure;
pub mod projections;
pub mod services;
pub mod nats;
pub mod cross_domain;

// Quality dimension module for Gärdenfors conceptual spaces
pub mod quality;

// Re-export cim-domain-spaces types we extend
pub use cim_domain_spaces::{
    // Conceptual space foundation
    ConceptualSpace, ConceptualSpaceId, Concept, ConceptId,
    // Quality dimensions
    QualityDimension, QualityType,
    // Geometric types for quality space
    Point3, Vector3, UnitVector3,
    // Knowledge levels
    KnowledgeLevel, EvidenceScore,
    // Voronoi for relationship clustering
    VoronoiTessellation, VoronoiCell,
};

// Re-export main types
pub use aggregates::{EdgeConcept, HyperEdgeConcept, RelationshipSpace};
pub use value_objects::{
    EntityRef, EntityType, RelationshipId, RelationshipCategory,
    ValidityPeriod, IncidenceMatrix, ParticipantRole, Formality,
};
pub use events::RelationshipEvent;
pub use commands::RelationshipCommand;
pub use quality::{RelationshipQuality, QualityPoint};

// Domain-specific error types
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RelationshipError {
    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("Invalid relationship: {0}")]
    InvalidRelationship(String),

    #[error("Quality dimension out of range: {0}")]
    QualityOutOfRange(String),

    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("Hyperedge requires at least 2 participants")]
    InsufficientParticipants,

    #[error("CID resolution failed: {0}")]
    CidResolutionFailed(String),

    #[error("Cross-domain event failed: {0}")]
    CrossDomainEventFailed(String),

    #[error("Space error: {0}")]
    SpaceError(#[from] cim_domain_spaces::SpaceError),
}

pub type RelationshipResult<T> = Result<T, RelationshipError>;

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_imports() {
        // Verify cim-domain-spaces types are accessible
        let _space_id = ConceptualSpaceId::new();
        let _concept_id = ConceptId::new();
        let _point = Point3::new(0.5, 0.7, 0.3); // quality dimensions
        let _level = KnowledgeLevel::Unknown;
    }
}
