/*
 * Copyright (c) 2025 - Cowboy AI, LLC.
 */

//! Quality Dimensions for Relationship Conceptual Space
//!
//! Following Gärdenfors' Conceptual Spaces theory, relationships exist in a
//! multi-dimensional quality space where they can be compared, measured, and
//! clustered based on their quality dimensions.
//!
//! ## Core Quality Dimensions for Relationships
//!
//! - **Strength**: How strong/weak the relationship is (0.0 - 1.0)
//! - **Trust**: Level of trust between entities (0.0 - 1.0)
//! - **Formality**: Informal -> Legal scale
//! - **Duration**: Temporal extent of the relationship
//! - **Reciprocity**: How mutual/one-sided the relationship is (0.0 - 1.0)
//!
//! ## Quality Space
//!
//! Each relationship occupies a point in this 5-dimensional quality space.
//! Similar relationships cluster together, enabling:
//! - Similarity queries ("find relationships like X")
//! - Clustering ("group similar relationships")
//! - Voronoi tessellation ("define relationship neighborhoods")

use crate::value_objects::{Formality, ValidityPeriod};
use serde::{Deserialize, Serialize};

/// Quality point in the 5-dimensional relationship space
///
/// Represents a relationship's position in the conceptual quality space.
/// Each dimension is normalized to [0.0, 1.0] for consistent distance calculations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct QualityPoint {
    /// Strength dimension (0.0 = weak, 1.0 = strong)
    pub strength: f64,
    /// Trust dimension (0.0 = no trust, 1.0 = complete trust)
    pub trust: f64,
    /// Formality dimension (0.0 = informal, 1.0 = legal)
    pub formality: f64,
    /// Duration dimension (0.0 = instantaneous, 1.0 = permanent)
    pub duration: f64,
    /// Reciprocity dimension (0.0 = one-sided, 1.0 = fully mutual)
    pub reciprocity: f64,
}

impl QualityPoint {
    /// Create a new quality point
    pub fn new(strength: f64, trust: f64, formality: f64, duration: f64, reciprocity: f64) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            trust: trust.clamp(0.0, 1.0),
            formality: formality.clamp(0.0, 1.0),
            duration: duration.clamp(0.0, 1.0),
            reciprocity: reciprocity.clamp(0.0, 1.0),
        }
    }

    /// Create a quality point with all dimensions at the origin
    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Create a quality point with default values for a category
    pub fn default_for_employment() -> Self {
        Self::new(0.7, 0.6, 0.75, 0.8, 0.6) // Strong, medium trust, contractual, long-term
    }

    pub fn default_for_friendship() -> Self {
        Self::new(0.5, 0.8, 0.0, 0.5, 0.9) // Medium strength, high trust, informal, mutual
    }

    pub fn default_for_membership() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.6, 0.5) // Balanced across dimensions
    }

    /// Calculate Euclidean distance to another point
    pub fn distance(&self, other: &Self) -> f64 {
        let ds = self.strength - other.strength;
        let dt = self.trust - other.trust;
        let df = self.formality - other.formality;
        let dd = self.duration - other.duration;
        let dr = self.reciprocity - other.reciprocity;

        (ds * ds + dt * dt + df * df + dd * dd + dr * dr).sqrt()
    }

    /// Calculate weighted distance (some dimensions matter more)
    pub fn weighted_distance(&self, other: &Self, weights: &QualityWeights) -> f64 {
        let ds = (self.strength - other.strength) * weights.strength;
        let dt = (self.trust - other.trust) * weights.trust;
        let df = (self.formality - other.formality) * weights.formality;
        let dd = (self.duration - other.duration) * weights.duration;
        let dr = (self.reciprocity - other.reciprocity) * weights.reciprocity;

        (ds * ds + dt * dt + df * df + dd * dd + dr * dr).sqrt()
    }

    /// Linear interpolation toward another point
    pub fn lerp(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            self.strength + (other.strength - self.strength) * t,
            self.trust + (other.trust - self.trust) * t,
            self.formality + (other.formality - self.formality) * t,
            self.duration + (other.duration - self.duration) * t,
            self.reciprocity + (other.reciprocity - self.reciprocity) * t,
        )
    }

    /// Convert to array for nalgebra operations
    pub fn to_array(&self) -> [f64; 5] {
        [
            self.strength,
            self.trust,
            self.formality,
            self.duration,
            self.reciprocity,
        ]
    }

    /// Create from array
    pub fn from_array(arr: [f64; 5]) -> Self {
        Self::new(arr[0], arr[1], arr[2], arr[3], arr[4])
    }

    /// Convert to cim-domain-spaces Point3 (using first 3 dimensions)
    /// Useful for visualization and Voronoi tessellation
    pub fn to_point3(&self) -> cim_domain_spaces::Point3<f64> {
        cim_domain_spaces::Point3::new(self.strength, self.trust, self.formality)
    }
}

impl Default for QualityPoint {
    fn default() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.5, 0.5) // Center of quality space
    }
}

/// Weights for quality dimensions in distance calculations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct QualityWeights {
    pub strength: f64,
    pub trust: f64,
    pub formality: f64,
    pub duration: f64,
    pub reciprocity: f64,
}

impl Default for QualityWeights {
    fn default() -> Self {
        Self {
            strength: 1.0,
            trust: 1.0,
            formality: 1.0,
            duration: 1.0,
            reciprocity: 1.0,
        }
    }
}

impl QualityWeights {
    /// Weights emphasizing trust and strength
    pub fn trust_focused() -> Self {
        Self {
            strength: 1.5,
            trust: 2.0,
            formality: 0.5,
            duration: 0.5,
            reciprocity: 1.0,
        }
    }

    /// Weights emphasizing formality and duration (business focus)
    pub fn business_focused() -> Self {
        Self {
            strength: 1.0,
            trust: 1.0,
            formality: 2.0,
            duration: 1.5,
            reciprocity: 0.5,
        }
    }

    /// Weights emphasizing reciprocity (social focus)
    pub fn social_focused() -> Self {
        Self {
            strength: 1.0,
            trust: 1.5,
            formality: 0.5,
            duration: 0.5,
            reciprocity: 2.0,
        }
    }
}

/// Full relationship quality with value object representations
///
/// This is the high-level quality type that includes both normalized
/// QualityPoint values and the original value objects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipQuality {
    /// Strength of the relationship (0.0 - 1.0)
    pub strength: f64,

    /// Trust level (0.0 - 1.0)
    pub trust: f64,

    /// Formality level
    pub formality: Formality,

    /// Temporal validity
    pub duration: ValidityPeriod,

    /// Reciprocity level (0.0 - 1.0)
    pub reciprocity: f64,
}

impl RelationshipQuality {
    /// Create a new relationship quality
    pub fn new(
        strength: f64,
        trust: f64,
        formality: Formality,
        duration: ValidityPeriod,
        reciprocity: f64,
    ) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            trust: trust.clamp(0.0, 1.0),
            formality,
            duration,
            reciprocity: reciprocity.clamp(0.0, 1.0),
        }
    }

    /// Convert to normalized QualityPoint
    pub fn to_quality_point(&self) -> QualityPoint {
        // Normalize duration based on whether it's ongoing and how long
        let duration_normalized = if self.duration.has_ended() {
            // Ended relationships: normalize by how long they lasted
            self.duration
                .duration_days()
                .map(|days| (days as f64 / 365.0).min(1.0))
                .unwrap_or(0.0)
        } else {
            // Ongoing relationships: normalize by time since start
            let days = (chrono::Utc::now() - self.duration.starts_at).num_days();
            ((days as f64) / 365.0).min(1.0)
        };

        QualityPoint::new(
            self.strength,
            self.trust,
            self.formality.as_f64(),
            duration_normalized,
            self.reciprocity,
        )
    }

    /// Create default quality for employment relationships
    pub fn default_employment() -> Self {
        Self::new(
            0.7,
            0.6,
            Formality::Contractual,
            ValidityPeriod::ongoing_now(),
            0.6,
        )
    }

    /// Create default quality for friendship relationships
    pub fn default_friendship() -> Self {
        Self::new(
            0.5,
            0.8,
            Formality::Informal,
            ValidityPeriod::ongoing_now(),
            0.9,
        )
    }

    /// Create default quality for membership relationships
    pub fn default_membership() -> Self {
        Self::new(
            0.5,
            0.5,
            Formality::Formal,
            ValidityPeriod::ongoing_now(),
            0.5,
        )
    }
}

impl Default for RelationshipQuality {
    fn default() -> Self {
        Self::new(
            0.5,
            0.5,
            Formality::Formal,
            ValidityPeriod::ongoing_now(),
            0.5,
        )
    }
}

/// Quality dimension definition for the relationship conceptual space
///
/// Defines a single dimension in the quality space with bounds and semantics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipDimension {
    /// Dimension identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Minimum value (typically 0.0)
    pub min_value: f64,
    /// Maximum value (typically 1.0)
    pub max_value: f64,
    /// Description of what this dimension represents
    pub description: String,
    /// Labels for key points on the dimension
    pub labels: Vec<(f64, String)>,
}

impl RelationshipDimension {
    /// Create the strength dimension
    pub fn strength() -> Self {
        Self {
            id: "strength".to_string(),
            name: "Strength".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            description: "How strong or weak the relationship is".to_string(),
            labels: vec![
                (0.0, "Weak".to_string()),
                (0.5, "Moderate".to_string()),
                (1.0, "Strong".to_string()),
            ],
        }
    }

    /// Create the trust dimension
    pub fn trust() -> Self {
        Self {
            id: "trust".to_string(),
            name: "Trust".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            description: "Level of trust between entities".to_string(),
            labels: vec![
                (0.0, "No Trust".to_string()),
                (0.5, "Partial Trust".to_string()),
                (1.0, "Complete Trust".to_string()),
            ],
        }
    }

    /// Create the formality dimension
    pub fn formality() -> Self {
        Self {
            id: "formality".to_string(),
            name: "Formality".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            description: "Level of formality from informal to legal".to_string(),
            labels: vec![
                (0.0, "Informal".to_string()),
                (0.25, "Semi-Formal".to_string()),
                (0.5, "Formal".to_string()),
                (0.75, "Contractual".to_string()),
                (1.0, "Legal".to_string()),
            ],
        }
    }

    /// Create the duration dimension
    pub fn duration() -> Self {
        Self {
            id: "duration".to_string(),
            name: "Duration".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            description: "Temporal extent of the relationship".to_string(),
            labels: vec![
                (0.0, "Instantaneous".to_string()),
                (0.25, "Short-term".to_string()),
                (0.5, "Medium-term".to_string()),
                (0.75, "Long-term".to_string()),
                (1.0, "Permanent".to_string()),
            ],
        }
    }

    /// Create the reciprocity dimension
    pub fn reciprocity() -> Self {
        Self {
            id: "reciprocity".to_string(),
            name: "Reciprocity".to_string(),
            min_value: 0.0,
            max_value: 1.0,
            description: "How mutual or one-sided the relationship is".to_string(),
            labels: vec![
                (0.0, "One-sided".to_string()),
                (0.5, "Partially Mutual".to_string()),
                (1.0, "Fully Mutual".to_string()),
            ],
        }
    }

    /// Get all standard relationship dimensions
    pub fn all_dimensions() -> Vec<Self> {
        vec![
            Self::strength(),
            Self::trust(),
            Self::formality(),
            Self::duration(),
            Self::reciprocity(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_point_distance() {
        let p1 = QualityPoint::new(0.0, 0.0, 0.0, 0.0, 0.0);
        let p2 = QualityPoint::new(1.0, 0.0, 0.0, 0.0, 0.0);

        assert!((p1.distance(&p2) - 1.0).abs() < 0.001);

        let p3 = QualityPoint::new(1.0, 1.0, 1.0, 1.0, 1.0);
        let expected = (5.0_f64).sqrt(); // sqrt(1^2 + 1^2 + 1^2 + 1^2 + 1^2)
        assert!((p1.distance(&p3) - expected).abs() < 0.001);
    }

    #[test]
    fn test_quality_point_lerp() {
        let p1 = QualityPoint::new(0.0, 0.0, 0.0, 0.0, 0.0);
        let p2 = QualityPoint::new(1.0, 1.0, 1.0, 1.0, 1.0);

        let mid = p1.lerp(&p2, 0.5);
        assert!((mid.strength - 0.5).abs() < 0.001);
        assert!((mid.trust - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_relationship_quality_conversion() {
        let quality = RelationshipQuality::default_employment();
        let point = quality.to_quality_point();

        assert!((point.strength - 0.7).abs() < 0.001);
        assert!((point.trust - 0.6).abs() < 0.001);
        assert!((point.formality - 0.75).abs() < 0.001); // Contractual
    }

    #[test]
    fn test_quality_clamping() {
        let point = QualityPoint::new(2.0, -1.0, 0.5, 0.5, 0.5);
        assert_eq!(point.strength, 1.0);
        assert_eq!(point.trust, 0.0);
    }
}
