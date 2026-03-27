//! Mock client definition and builder methods.

use serde_json::Value;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::model::{
    AnalyticsSnapshot, ApiError, Context, ContextId, ErrorKind, File, FileEntry, FileId, Health,
    NewContext, NewFile, NewRun, Page, Pagination, RunDetail, RunResult, RunSummary,
};
use crate::service::RunQuery;

/// Boxed, thread-safe handler function.
pub(crate) type Handler<I, O> = Box<dyn Fn(I) -> Result<O> + Send + Sync>;

/// Returns an error for unconfigured methods.
pub(crate) fn not_configured<O>(method: &str) -> Result<O> {
    Err(Error::Api(ApiError {
        status: 501,
        kind: ErrorKind::NotImplemented,
        message: Some(format!("mock: {method} not configured")),
        ..Default::default()
    }))
}

/// Declares mock handler fields, `new()` defaults, and `on_*` builder methods.
macro_rules! mock_handlers {
    ($(
        $(#[$section:meta])*
        $name:ident : $input:ty => $output:ty
    ),* $(,)?) => {
        /// Mock Nvisy Runtime API client for testing.
        ///
        /// Each method defaults to returning an error until configured
        /// with the corresponding `on_*` builder method.
        pub struct MockRuntime {
            $( pub(crate) $name: Handler<$input, $output>, )*
        }

        impl MockRuntime {
            /// Creates a new mock with all methods returning "not configured" errors.
            pub fn new() -> Self {
                Self {
                    $( $name: Box::new(|_| not_configured(stringify!($name))), )*
                }
            }

            $(
                pub fn $name(mut self, f: impl Fn($input) -> Result<$output> + Send + Sync + 'static) -> Self {
                    self.$name = Box::new(f);
                    self
                }
            )*
        }

        impl Default for MockRuntime {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

mock_handlers! {
    // infra
    on_health:        () => Health,
    on_analytics:     () => AnalyticsSnapshot,
    on_openapi_spec:  () => Value,

    // files
    on_upload_file:   NewFile => FileId,
    on_download_file: Uuid => File,
    on_list_files:    Pagination => Page<FileEntry>,
    on_delete_file:   Uuid => (),
    on_delete_files:  () => (),

    // contexts
    on_upload_context:   NewContext => ContextId,
    on_download_context: Uuid => Context,
    on_list_contexts:    Pagination => Page<Uuid>,
    on_delete_context:   Uuid => (),
    on_delete_contexts:  () => (),

    // runs
    on_create_run:  NewRun => RunResult,
    on_list_runs:   (RunQuery, Pagination) => Page<RunSummary>,
    on_get_run:     Uuid => RunDetail,
    on_cancel_run:  Uuid => (),
    on_delete_run:  Uuid => (),
    on_delete_runs: () => (),
}
