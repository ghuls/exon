// Copyright 2023 WHERE TRUE Technologies.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(missing_docs)]

//! Exon is a library to facilitate open-ended analysis of scientific data, ease the application of ML models, and provide a common data interface for science and engineering teams.
//!
//! # Overview
//!
//! The main interface for users is through datafusion's `SessionContext` plus the [`ExonSessionExt`] extension trait. This has a number of convenience methods for loading data from various sources.
//!
//! See the `read_*` methods on [`ExonSessionExt`] for more information. For example, `read_fasta`, or `read_gff`. There's also a `read_inferred_exon_table` method that will attempt to infer the data type and compression from the file extension for ease of use.
//!
//! To facilitate those methods, Exon implements a number of traits for DataFusion that serve as a good base for scientific data work. See the [`datasources`] module for more information.
//!
//! [`ExonSessionExt`]: context::ExonSessionExt
//! [`datasources`]: datasources
//!
//! # Examples
//!
//! ## Loading a FASTQ file
//!
//! ```rust
//! use exon::ExonSessionExt;
//!
//! use datafusion::prelude::*;
//! use datafusion::error::Result;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ctx = SessionContext::new();
//!
//! let df = ctx.read_fastq("test-data/datasources/fastq/test.fastq", None).await?;
//!
//! assert_eq!(df.schema().fields().len(), 4);
//! assert_eq!(df.schema().field(0).name(), "name");
//! assert_eq!(df.schema().field(1).name(), "description");
//! assert_eq!(df.schema().field(2).name(), "sequence");
//! assert_eq!(df.schema().field(3).name(), "quality_scores");
//! # Ok(())
//! # }
//! ```
//!
//! ## Loading a ZSTD-compressed FASTA file
//!
//! ```rust
//! use exon::ExonSessionExt;
//!
//! use datafusion::prelude::*;
//! use datafusion::error::Result;
//! use datafusion::common::FileCompressionType;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let ctx = SessionContext::new();
//!
//! let file_compression = FileCompressionType::ZSTD;
//! let df = ctx.read_fasta("test-data/datasources/fasta/test.fasta.zstd", Some(file_compression)).await?;
//!
//! assert_eq!(df.schema().fields().len(), 3);
//! assert_eq!(df.schema().field(0).name(), "id");
//! assert_eq!(df.schema().field(1).name(), "description");
//! assert_eq!(df.schema().field(2).name(), "sequence");
//!
//! let results = df.collect().await?;
//! assert_eq!(results.len(), 1);  // 1 batch, small dataset
//!
//! # Ok(())
//! # }
//! ```

mod session_context;
pub use session_context::ExonSessionExt;

mod config;
pub use config::new_exon_config;

/// Data sources for Exon.
pub mod datasources;

/// UDFs for Exon.
pub mod udfs;

/// Physical plan augmentations for Exon.
pub mod physical_optimizer;

/// Logical plan augmentations for Exon.
pub mod logical_optimizer;

/// Physical plan augmentations for Exon.
pub mod physical_plan;

/// Utilities for moving data across the FFI boundary.
#[cfg(feature = "ffi")]
pub mod ffi;

/// Runtime environment for Exon.
mod runtime_env;

pub use runtime_env::ExonRuntimeEnvExt;

/// Error types for Exon.
pub mod error;

/// Utilities for working with stream bgzf files.
pub mod streaming_bgzf;

mod repartitionable;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    #[allow(unused_imports)]
    use datafusion::datasource::listing::ListingTableUrl;

    use datafusion::{
        logical_expr::Operator,
        physical_plan::{expressions::BinaryExpr, PhysicalExpr},
    };
    use object_store::{local::LocalFileSystem, ObjectStore};

    use std::sync::Once;

    static INIT: Once = Once::new();

    pub(crate) fn eq(left: Arc<dyn PhysicalExpr>, right: Arc<dyn PhysicalExpr>) -> BinaryExpr {
        BinaryExpr::new(left, Operator::Eq, right)
    }

    pub(crate) fn gteq(left: Arc<dyn PhysicalExpr>, right: Arc<dyn PhysicalExpr>) -> BinaryExpr {
        BinaryExpr::new(left, Operator::GtEq, right)
    }

    pub(crate) fn gt(left: Arc<dyn PhysicalExpr>, right: Arc<dyn PhysicalExpr>) -> BinaryExpr {
        BinaryExpr::new(left, Operator::Gt, right)
    }

    pub(crate) fn lt(left: Arc<dyn PhysicalExpr>, right: Arc<dyn PhysicalExpr>) -> BinaryExpr {
        BinaryExpr::new(left, Operator::Lt, right)
    }

    pub fn make_object_store() -> Arc<dyn ObjectStore> {
        let local_file_system = LocalFileSystem::new();

        Arc::new(local_file_system)
    }

    #[cfg(feature = "fixtures")]
    pub fn test_fixture_table_url(
        relative_path: &str,
    ) -> Result<ListingTableUrl, datafusion::error::DataFusionError> {
        use std::path::PathBuf;

        let cwd = std::env::current_dir().unwrap().join("exon");

        let start_directory = std::env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap_or(cwd);

        let abs_file_path = start_directory
            .join("test-data")
            .join("fixtures")
            .join(relative_path);

        ListingTableUrl::parse(abs_file_path.to_str().unwrap())
    }

    pub fn setup_tracing() {
        INIT.call_once(|| {
            let subscriber = tracing_subscriber::fmt()
                .with_max_level(tracing::Level::DEBUG)
                .finish();
            tracing::subscriber::set_global_default(subscriber)
                .expect("setting default subscriber failed");
        });
    }
}
