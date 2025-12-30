/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! Infrastructure for the Relationship Domain
//!
//! Event store, repositories, and NATS integration.

// Re-export from cim-domain-spaces infrastructure
pub use cim_domain_spaces::{
    EventStore, EventStoreError, RepositoryError, StoredEvent, EventMetadata,
};

// Placeholder for relationship-specific infrastructure
// TODO: Implement RelationshipEventStore, RelationshipRepository
