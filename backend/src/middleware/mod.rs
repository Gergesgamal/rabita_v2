pub mod auth_extractor;

/// Per-request context. `trace_id` is filled by the tracing layer once it is wired up.
pub struct RequestContext {
    pub trace_id: String,
}
