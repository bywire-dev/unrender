//! Evaluation harness for `unrender`.
//!
//! The repo exists to test one claim: that an unrendered tree serves a
//! consumer better than raw terminal output. No single number tests that, so
//! this crate provides the complementary measurements and the fixture
//! discovery they share. Notably, [`score`] and `fidelity` answer genuinely
//! different questions — see the module docs on each.

pub mod baseline;
pub mod fixtures;
pub mod score;

pub use fixtures::{discover, workspace_root, Fixture};
pub use score::{compute_score, ScoreResult, SelectionCheck};
