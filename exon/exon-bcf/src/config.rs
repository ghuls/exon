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

use std::sync::Arc;

use arrow::datatypes::SchemaRef;
use exon_common::DEFAULT_BATCH_SIZE;
use object_store::ObjectStore;

/// Configuration for a BCF datasource.
pub struct BCFConfig {
    /// The object store to use for reading BCF files.
    pub object_store: Arc<dyn ObjectStore>,

    /// The number of records to read at a time.
    pub batch_size: usize,

    /// The file schema to use.
    pub file_schema: Arc<arrow::datatypes::Schema>,

    /// Any projections to apply to the resulting batches.
    pub projection: Option<Vec<usize>>,
}

impl BCFConfig {
    /// Create a new BCF configuration.
    pub fn new(object_store: Arc<dyn ObjectStore>, file_schema: SchemaRef) -> Self {
        Self {
            object_store,
            batch_size: DEFAULT_BATCH_SIZE,
            file_schema,
            projection: None,
        }
    }

    /// Set the batch size.
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Set the projection.
    pub fn with_projection(mut self, projection: Vec<usize>) -> Self {
        self.projection = Some(projection);
        self
    }

    /// Set the projection from an optional vector.
    pub fn with_some_projection(mut self, projection: Option<Vec<usize>>) -> Self {
        self.projection = projection;
        self
    }

    /// Get the projection, returning the identity projection if none is set.
    pub fn projection(&self) -> Vec<usize> {
        self.projection
            .clone()
            .unwrap_or_else(|| (0..self.file_schema.fields().len()).collect())
    }

    /// Get the projected schema.
    pub fn projected_schema(&self) -> arrow::error::Result<SchemaRef> {
        let schema = self.file_schema.project(&self.projection())?;

        Ok(Arc::new(schema))
    }
}
