pub trait UseCaseDescriptor {
    /// Stable identity
    const NAME: &'static str;

    /// Authorization identity
    const RESOURCE: &'static str;
    const ACTION: &'static str;

    /// Observability
    const AUDIT: bool = true;
    const METRICS: bool = true;

    /// Operational hints
    const RATE_LIMITED: bool = false;
    const CACHEABLE: bool = false;
}
