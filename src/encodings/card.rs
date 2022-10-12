//! # CNF Encodings for Cardinality Constraints
//!
//! The module contains implementations of CNF encodings for cardinality
//! constraints. It defines traits for (non-)incremental cardinality constraints
//! and encodings implementing these traits.

use super::{BoundType, EncodingError};
use crate::{
    instances::{ManageVars, CNF},
    types::Lit,
};

mod totalizer;
pub use totalizer::Totalizer;

/// Trait for types that can encode a cardinality constraint of form `sum of
/// lits <> rhs` where `<>` is either `>=`, `<=` or both.
pub trait EncodeCard: Sized {
    /// Constructs a new cardinality encoding. If the given bound type is not
    /// supported by the implementing type, it returns
    /// [`EncodingError::NoTypeSupport`].
    fn new(bound_type: BoundType) -> Result<Self, EncodingError>;
    /// Constructs a new cardinality encoding from input literals. If the given
    /// bound type is not supported by the implementing type, it returns
    /// [`EncodingError::NoTypeSupport`].
    fn new_from_lits<CE: EncodeCard>(
        lits: Vec<Lit>,
        bound_type: BoundType,
    ) -> Result<CE, EncodingError> {
        let mut ce = CE::new(bound_type)?;
        ce.add(lits);
        Ok(ce)
    }
    /// Adds new literals to the cardinality encoding
    fn add(&mut self, lits: Vec<Lit>);
    /// Lazily encodes the cardinality constraint for `rhs` values at most
    /// `max_rhs` and at least `min_rhs`. `var_manager` is the variable manager
    /// to use for tracking new variables. A specific encoding might ignore
    /// `min_rhs` or `max_rhs`. Returns [`EncodingError::InvalidBounds`] if the
    /// bounds are invalid.
    fn encode<VM: ManageVars>(
        &mut self,
        min_rhs: usize,
        max_rhs: usize,
        var_manager: &mut VM,
    ) -> Result<CNF, EncodingError>;
    /// Returns assumptions for enforcing an upper bound (`sum of lits <= ub`)
    /// or an error if the encoding does not support upper bounding. Make sure
    /// that nothing was added to the encoding between the last call to
    /// [`EncodeCard::encode`] and this method, otherwise
    /// [`super::EncodingError::NotEncoded`] will be returned.
    fn enforce_ub(&self, ub: usize) -> Result<Vec<Lit>, EncodingError>;
    /// Returns assumptions for enforcing a lower bound (`sum of lits >= lb`) or an
    /// error if the encoding does not support lower bounding. Make sure that
    /// nothing was added to the encoding between the last call to
    /// [`EncodeCard::encode`] and this method, otherwise
    /// [`super::EncodingError::NotEncoded`] will be returned.
    fn enforce_lb(&self, lb: usize) -> Result<Vec<Lit>, EncodingError>;
    /// Returns assumptions for enforcing an equality (`sum of lits = b`) or an
    /// error if the encoding does not support one of the two required bound
    /// types. Make sure that nothing was added to the encoding between the last
    /// call to [`EncodeCard::encode`] and this method, otherwise
    /// [`super::EncodingError::NotEncoded`] will be returned.
    fn enforce_eq(&self, b: usize) -> Result<Vec<Lit>, EncodingError> {
        let mut assumps = self.enforce_ub(b)?;
        assumps.extend(self.enforce_lb(b)?);
        Ok(assumps)
    }
}

pub trait IncEncodeCard: EncodeCard {
    /// Constructs a new cardinality encoding that reserves all variables on the
    /// first call to an encode method. If the given bound type is not supported
    /// by the implementing type, it returns [`EncodingError::NoTypeSupport`].
    fn new_reserving(bound_type: BoundType) -> Result<Self, EncodingError>;
    /// Constructs a new cardinality encoding that reserves all variables on the
    /// first call to an encode method from input literals. If the given bound
    /// type is not supported by the implementing type, it returns
    /// [`EncodingError::NoTypeSupport`].
    fn new_from_lits<ICE: IncEncodeCard>(
        lits: Vec<Lit>,
        bound_type: BoundType,
    ) -> Result<ICE, EncodingError> {
        let mut ce = ICE::new_reserving(bound_type)?;
        ce.add(lits);
        Ok(ce)
    }
    /// Lazily encodes a change in the cardinality constraint for `rhs` values
    /// at most `max_rhs` and at least `min_rhs`. A change can be added literals
    /// or changed bounds. `var_manager` is the variable manager to use for
    /// tracking new variables. The returned CNF might be empty if no change
    /// needs to be encoded. A specific encoding might (have to) ignore
    /// `min_rhs` or `max_rhs`. Returns [`EncodingError::InvalidBounds`] if the
    /// bounds are invalid.
    fn encode_change<VM: ManageVars>(
        &mut self,
        min_rhs: usize,
        max_rhs: usize,
        var_manager: &mut VM,
    ) -> Result<CNF, EncodingError>;
}
