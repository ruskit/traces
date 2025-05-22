use crate::errors::TracesError;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing::info;

pub fn install() -> Result<SdkTracerProvider, TracesError> {
    info!("traces::install noop tracer installed");

    Ok(SdkTracerProvider::default())
}
