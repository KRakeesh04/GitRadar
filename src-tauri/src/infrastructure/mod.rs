pub mod database;
pub mod git;
pub mod filesystem;

// Infrastructure layer: adapters and implementations for external systems
// This layer handles all interactions with external resources (databases, file systems, git repos)
// and provides clean abstractions for the domain and services layers to use.
