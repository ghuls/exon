// Copyright 2024 WHERE TRUE Technologies.
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

mod array_builder;
mod async_batch_stream;
mod config;
mod indexed_async_batch_stream;
mod object_store_fasta_repository_adapter;

/// CRAM configuration struct.
pub use config::CRAMConfig;

/// CRAM Batch Stream.
pub use async_batch_stream::AsyncBatchStream;

/// Indexed CRAM Batch Stream.
pub use indexed_async_batch_stream::IndexedAsyncBatchStream;

/// Object store repository adapter.
pub use object_store_fasta_repository_adapter::ObjectStoreFastaRepositoryAdapter;
