//! # CRAbE_framework
//!
//! This crate provides shared utilities and settings for the CRAbE project.
//!
//! The `config` module contains settings that are common to multiple CRAbE
//! crates.
//! The `constant` module contains constants that are common to multiple
//! CRAbE crates.
//! The `component` module contains traits and structs that defines the component architecture
//! used in the CRAbE project
//!
//! This crate is intended to be used as a dependency by other CRAbE crates, and
//! should not be used on its own. Please refer to the individual modules for
//! more information and usage instructions.

/// This module contains the structure of the configuration settings that are
/// shared across multiple other crates in the CRAbE project. These settings are
/// meant to be accessed and used by other CRAbE crates as a way of maintaining
/// consistency across the project. Please refer to the documentation of
/// individual settings for more information and usage instructions.
pub mod config;

/// This module contains traits and structs that defines the component
/// architecture used in the CRAbE project.
/// Components are building blocks that perform a specific task or set of related tasks,
/// and can be combined to form a pipeline for processing robot soccer data.
/// The current components include an input component for receiving data,
/// a filter component for processing and filtering the data,
/// a decision component for making game decisions,
/// a tools component for assisting in those decisions,
/// a guard component for ensuring safe operation,
/// and an output component for sending commands to the robots.
///
/// New components can be added by creating additional crates that implement the
/// necessary traits. For example, while the current implementation of
/// crabe_decision uses a manager system, it is possible to implement the
/// decision-making component using machine learning techniques by creating a
/// new crate that implements the DecisionComponent trait.
pub mod component;

/// TODO
pub mod data;

/// This module contains constants that are used throughout the CRAbE project.
/// These constants are meant to be accessed and used by other CRAbE crates as a
/// way of maintaining consistency across the project.
pub mod constant;
