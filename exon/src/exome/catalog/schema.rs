use std::{collections::HashMap, str::FromStr, sync::Arc};

use async_trait::async_trait;
use datafusion::{
    catalog::schema::SchemaProvider, common::FileCompressionType, datasource::TableProvider,
    prelude::SessionContext,
};

use crate::{
    datasources::{ExonFileType, ExonListingTableFactory},
    exome::proto,
    ExonRuntimeEnvExt,
};

use super::ExomeCatalogClient;

pub struct Schema {
    inner: proto::Schema,
    exome_client: ExomeCatalogClient,
    session_context: Arc<SessionContext>,
    tables: HashMap<String, proto::Table>,
}

// Create a debug implementation for Schema
impl std::fmt::Debug for Schema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Schema")
            .field("inner", &self.inner)
            .finish()
    }
}

impl Schema {
    pub async fn new(
        proto_schema: proto::Schema,
        session_context: Arc<SessionContext>,
        client: ExomeCatalogClient,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut s = Self {
            inner: proto_schema,
            exome_client: client,
            session_context,
            tables: HashMap::new(),
        };

        s.refresh().await?;

        Ok(s)
    }

    pub async fn refresh(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let schema_id = self.inner.id.clone();

        let tables = self.exome_client.get_tables(schema_id).await?;

        self.tables.clear();

        for table in tables {
            self.tables.insert(table.name.clone(), table);
        }

        Ok(())
    }
}

#[async_trait]
impl SchemaProvider for Schema {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn table_names(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }

    async fn table(&self, name: &str) -> Option<Arc<dyn TableProvider>> {
        let proto_table = self.tables.get(name)?;

        self.session_context
            .runtime_env()
            .exon_register_object_store_uri(&proto_table.location.clone())
            .await
            .unwrap();

        let file_compression_type =
            FileCompressionType::from_str(&proto_table.compression_type_id).unwrap();

        let file_type = ExonFileType::from_str(&proto_table.file_format).unwrap();

        let table_provider_factory = ExonListingTableFactory {};
        let table_provider = table_provider_factory
            .create_from_file_type(
                &self.session_context.state(),
                file_type,
                file_compression_type,
                proto_table.location.clone(),
            )
            .await
            .unwrap();

        Some(table_provider)
    }

    fn table_exist(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }
}
