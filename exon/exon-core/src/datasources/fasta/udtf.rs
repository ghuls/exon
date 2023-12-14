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

use datafusion::{
    datasource::{function::TableFunctionImpl, TableProvider},
    error::Result,
    logical_expr::Expr,
};
use exon_fasta::new_fasta_schema_builder;

use crate::datasources::ScanFunction;

use super::table_provider::{ListingFASTATable, ListingFASTATableConfig, ListingFASTATableOptions};

/// A table function that returns a table provider for a FASTA file.
#[derive(Debug, Default)]
pub struct FastaScanFunction {}

impl TableFunctionImpl for FastaScanFunction {
    fn call(&self, exprs: &[Expr]) -> Result<Arc<dyn TableProvider>> {
        let listing_scan_function = ScanFunction::try_from(exprs)?;

        let fasta_schema = new_fasta_schema_builder().build();

        let listing_table_options =
            ListingFASTATableOptions::new(listing_scan_function.file_compression_type);

        let listing_fasta_table_config =
            ListingFASTATableConfig::new(listing_scan_function.listing_table_url)
                .with_options(listing_table_options);

        let listing_fasta_table =
            ListingFASTATable::try_new(listing_fasta_table_config, fasta_schema)?;

        Ok(Arc::new(listing_fasta_table))
    }
}