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

use std::{any::Any, fmt::Display, hash::Hash, sync::Arc};

use arrow::datatypes::SchemaRef;
use datafusion::{
    error::{DataFusionError, Result},
    physical_plan::{expressions::BinaryExpr, PhysicalExpr},
};
use noodles::core::Region;

use super::{
    chrom_physical_expr::ChromPhysicalExpr, interval_physical_expr::IntervalPhysicalExpr,
    InvalidRegionError,
};

#[derive(Debug)]
pub struct RegionPhysicalExpr {
    region: Region,
    interval_expr: Arc<dyn PhysicalExpr>,
    chrom_expr: Arc<dyn PhysicalExpr>,
}

impl RegionPhysicalExpr {
    pub fn new(
        region: Region,
        interval_expr: Arc<dyn PhysicalExpr>,
        chrom_expr: Arc<dyn PhysicalExpr>,
    ) -> Self {
        Self {
            region,
            interval_expr,
            chrom_expr,
        }
    }

    pub fn region(&self) -> &Region {
        &self.region
    }

    #[allow(dead_code)]
    pub fn from_region(region: Region, schema: SchemaRef) -> Result<Self> {
        let interval_expr = IntervalPhysicalExpr::from_interval(region.interval(), &schema)?;
        let chrom_expr = ChromPhysicalExpr::from_chrom(region.name(), &schema)?;

        let region_expr = Self::new(region, Arc::new(interval_expr), Arc::new(chrom_expr));

        Ok(region_expr)
    }
}

impl Display for RegionPhysicalExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RegionPhysicalExpr {{ region: {} }}", self.region)
    }
}

impl TryFrom<BinaryExpr> for RegionPhysicalExpr {
    type Error = DataFusionError;

    fn try_from(expr: BinaryExpr) -> Result<Self, Self::Error> {
        let chrom_op = expr
            .left()
            .as_any()
            .downcast_ref::<BinaryExpr>()
            .map(|e| ChromPhysicalExpr::try_from(e.clone()))
            .transpose()?;

        let pos_op = expr
            .right()
            .as_any()
            .downcast_ref::<BinaryExpr>()
            .map(|binary_expr| IntervalPhysicalExpr::try_from(binary_expr.clone()))
            .transpose()?;

        match (chrom_op, pos_op) {
            (Some(chrom), Some(pos)) => {
                let region = Region::new(chrom.chrom(), *pos.interval());
                Ok(Self::new(region, Arc::new(pos), Arc::new(chrom)))
            }
            (_, _) => Err(DataFusionError::External(InvalidRegionError.into())),
        }
    }
}

impl TryFrom<Arc<dyn PhysicalExpr>> for RegionPhysicalExpr {
    type Error = DataFusionError;

    fn try_from(expr: Arc<dyn PhysicalExpr>) -> Result<Self, Self::Error> {
        if let Some(binary_expr) = expr.as_any().downcast_ref::<BinaryExpr>() {
            Self::try_from(binary_expr.clone())
        } else {
            Err(DataFusionError::External(InvalidRegionError.into()))
        }
    }
}

impl PartialEq<dyn Any> for RegionPhysicalExpr {
    fn eq(&self, other: &dyn Any) -> bool {
        if let Some(other) = other.downcast_ref::<RegionPhysicalExpr>() {
            self.region == other.region
        } else {
            false
        }
    }
}

impl PhysicalExpr for RegionPhysicalExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn data_type(
        &self,
        _input_schema: &arrow::datatypes::Schema,
    ) -> datafusion::error::Result<arrow::datatypes::DataType> {
        Ok(arrow::datatypes::DataType::Boolean)
    }

    fn nullable(
        &self,
        _input_schema: &arrow::datatypes::Schema,
    ) -> datafusion::error::Result<bool> {
        Ok(true)
    }

    fn evaluate(
        &self,
        batch: &arrow::record_batch::RecordBatch,
    ) -> datafusion::error::Result<datafusion::physical_plan::ColumnarValue> {
        let binary_expr = BinaryExpr::new(
            self.chrom_expr.clone(),
            datafusion::logical_expr::Operator::And,
            self.interval_expr.clone(),
        );

        binary_expr.evaluate(batch)
    }

    fn children(&self) -> Vec<std::sync::Arc<dyn PhysicalExpr>> {
        vec![]
    }

    fn with_new_children(
        self: std::sync::Arc<Self>,
        _children: Vec<std::sync::Arc<dyn PhysicalExpr>>,
    ) -> datafusion::error::Result<std::sync::Arc<dyn PhysicalExpr>> {
        Ok(Arc::new(RegionPhysicalExpr::new(
            self.region().clone(),
            self.interval_expr.clone(),
            self.chrom_expr.clone(),
        )))
    }

    fn dyn_hash(&self, state: &mut dyn std::hash::Hasher) {
        let mut s = state;
        self.region.name().to_string().hash(&mut s);

        self.region.interval().start().hash(&mut s);
        self.region.interval().end().hash(&mut s);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::{array::BooleanArray, record_batch::RecordBatch};
    use datafusion::{
        logical_expr::Operator,
        physical_plan::{
            expressions::{col, lit, BinaryExpr},
            PhysicalExpr,
        },
        scalar::ScalarValue,
    };
    use noodles::core::{Position, Region};

    #[test]
    fn test_from_binary_exprs() {
        let schema = Arc::new(arrow::datatypes::Schema::new(vec![
            arrow::datatypes::Field::new("chrom", arrow::datatypes::DataType::Utf8, false),
            arrow::datatypes::Field::new("pos", arrow::datatypes::DataType::Int64, false),
        ]));

        let expr = BinaryExpr::new(
            Arc::new(BinaryExpr::new(
                col("chrom", &schema).unwrap(),
                Operator::Eq,
                lit(ScalarValue::from("1")),
            )),
            Operator::And,
            Arc::new(BinaryExpr::new(
                col("pos", &schema).unwrap(),
                Operator::Eq,
                lit(ScalarValue::from(4)),
            )),
        );

        let region = super::RegionPhysicalExpr::try_from(expr).unwrap();

        assert_eq!(
            region.region(),
            &Region::new(
                "1",
                noodles::core::region::Interval::from(
                    Position::new(4).unwrap()..=Position::new(4).unwrap()
                )
            )
        );
    }

    #[tokio::test]
    async fn test_evaluate() {
        let batch = RecordBatch::try_new(
            Arc::new(arrow::datatypes::Schema::new(vec![
                arrow::datatypes::Field::new("chrom", arrow::datatypes::DataType::Utf8, false),
                arrow::datatypes::Field::new("pos", arrow::datatypes::DataType::Int64, false),
            ])),
            vec![
                Arc::new(arrow::array::StringArray::from(vec![
                    "chr1", "chr1", "chr2",
                ])),
                Arc::new(arrow::array::Int64Array::from(vec![1, 2, 3])),
            ],
        )
        .unwrap();

        let region = "chr1:1-1".parse::<Region>().unwrap();

        let expr = super::RegionPhysicalExpr::from_region(region, batch.schema()).unwrap();

        let result = match expr.evaluate(&batch).unwrap() {
            datafusion::physical_plan::ColumnarValue::Array(array) => array,
            _ => panic!("Expected array"),
        };

        // Convert the result to a boolean array
        let result = result
            .as_any()
            .downcast_ref::<arrow::array::BooleanArray>()
            .unwrap();

        let expected = BooleanArray::from(vec![Some(true), Some(false), Some(false)]);

        result
            .iter()
            .zip(expected.iter())
            .for_each(|(result, expected)| {
                assert_eq!(result, expected);
            });
    }
}