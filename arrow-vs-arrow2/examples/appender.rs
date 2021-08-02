use std::sync::Arc;

// use arrow2::arrow::record_batch::RecordBatch;
use arrow2::{array::*, io::ipc::write::common::IpcWriteOptions, record_batch::RecordBatch};
use arrow_flight::utils::flight_data_from_arrow_batch;
use arrow_flight::utils::flight_data_to_arrow_batch;

type LargeUtf8Array = Utf8Array<i64>;
fn main() {
    let col0: ArrayRef = Arc::new(Int64Array::from_values(vec![0, 1, 2]));
    let col1: ArrayRef = Arc::new(LargeUtf8Array::from_iter_values(
        vec!["str1", "str2", "str3"].iter(),
    ));
    let batch = RecordBatch::try_from_iter(vec![("col0", col0), ("col1", col1)]).unwrap();
    let schema = batch.schema();

    let default_ipc_write_opt = IpcWriteOptions::default();

    let data = flight_data_from_arrow_batch(&batch, &default_ipc_write_opt).1;
    let batch_result = flight_data_to_arrow_batch(&data, schema.clone(), false, &[]);
    assert!(batch_result.is_ok());
}
