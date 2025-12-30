/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! Aggregates for the Relationship Domain
//!
//! ## Core Aggregates
//!
//! - **EdgeConcept**: Binary relationship between two entities, extending Concept
//! - **HyperEdgeConcept**: N-ary relationship among multiple entities
//! - **RelationshipSpace**: ConceptualSpace specialized for relationships
//!
//! All aggregates follow pure functional event sourcing with Mealy state machines.

mod edge;
mod hyperedge;
mod space;

pub use edge::{EdgeConcept, EdgeState};
pub use hyperedge::{HyperEdgeConcept, HyperEdgeState};
pub use space::RelationshipSpace;
