use configs::{app::AppConfigs, otlp::OTLPConfigs};
use opentelemetry_sdk::trace::Sampler;

pub(crate) fn get_sampler(app: &AppConfigs, otlp: &OTLPConfigs) -> Sampler {
    if app.env.is_local() {
        return Sampler::AlwaysOn;
    }

    let sampler = Sampler::TraceIdRatioBased(otlp.exporter_rate_base);
    return Sampler::ParentBased(Box::new(sampler));
}
