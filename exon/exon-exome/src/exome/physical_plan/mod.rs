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

use arrow::datatypes::{DataType, Field, Schema};
use datafusion::common::DFSchemaRef;
use once_cell::sync::Lazy;

mod create_catalog_exec;
mod create_schema_exec;
mod create_table_exec;
mod drop_catalog_exec;

pub(crate) use create_catalog_exec::CreateCatalogExec;
pub(crate) use create_schema_exec::CreateSchemaExec;
pub(crate) use create_table_exec::CreateTableExec;
pub(crate) use drop_catalog_exec::DropCatalogExec;

pub static CHANGE_SCHEMA: Lazy<Arc<Schema>> = Lazy::new(|| {
    Arc::new(Schema::new(vec![Field::new(
        "change",
        DataType::Utf8,
        false,
    )]))
});

pub static CHANGE_LOGICAL_SCHEMA: Lazy<DFSchemaRef> =
    Lazy::new(|| Arc::new(CHANGE_SCHEMA.as_ref().clone().try_into().unwrap()));