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

//! Data source for mzML files.

mod array_builder;
mod batch_reader;
mod config;
mod file_opener;
mod mzml_reader;
mod scanner;

/// Table provider for mzML files.
pub mod table_provider;

pub use config::MzMLConfig;
pub use file_opener::MzMLOpener;
pub use scanner::MzMLScan;

mod udtf;
pub use udtf::MzMLScanFunction;
