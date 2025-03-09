#![allow(clippy::unused_async,
    clippy::needless_raw_string_hashes,
    clippy::similar_names,
    clippy::inconsistent_struct_constructor,
    clippy::unnecessary_wraps,
    clippy::doc_markdown,
    clippy::unused_self,
    clippy::redundant_closure_for_method_calls,
    clippy::single_match_else,
    clippy::uninlined_format_args,
    clippy::cast_possible_truncation,
    clippy::must_use_candidate,
    clippy::match_same_arms,
    clippy::missing_errors_doc,
    clippy::too_many_lines,
    clippy::needless_pass_by_value,
    clippy::wildcard_imports,
    clippy::manual_let_else,
    clippy::map_unwrap_or,
    clippy::implicit_clone,
    clippy::if_not_else,
    clippy::trivially_copy_pass_by_ref,
    clippy::default_trait_access,
    clippy::struct_field_names,
    clippy::items_after_statements,
    clippy::manual_string_new,
    clippy::implicit_hasher,
    clippy::ref_option,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::redundant_else,
)]

pub mod cache;
pub mod clickhouse;
pub mod clickhouse_migration_manager;
pub mod config_parser; // TensorZero config file
pub mod embeddings; // embedding inference
pub mod endpoints; // API endpoints
pub mod error; // error handling
pub mod evals; // evaluation
pub mod function; // types and methods for working with TensorZero functions
pub mod gateway_util; // utilities for gateway
pub mod inference; // model inference
pub mod jsonschema_util; // utilities for working with JSON schemas
mod minijinja_util; // utilities for working with MiniJinja templates
pub mod model; // types and methods for working with TensorZero-supported models
pub mod model_table;
pub mod observability; // utilities for observability (logs, metrics, etc.)
mod testing;
pub mod tool; // types and methods for working with TensorZero tools
mod uuid_util; // utilities for working with UUIDs
mod variant; // types and methods for working with TensorZero variants
