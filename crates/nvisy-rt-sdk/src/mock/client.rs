//! Mock client definition and builder methods.

use serde_json::Value;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::model::{
    AnalyticsSnapshot, ApiError, Context, ContextEntry, ContextId, ErrorKind, File, FileEntry,
    FileId, Health, NewContext, NewFile, NewRun, Page, Pagination, RunDetail, RunResult,
    RunSummary,
};
use crate::service::RunQuery;

/// Boxed, thread-safe handler function.
pub(crate) type Handler<I, O> = Box<dyn Fn(I) -> Result<O> + Send + Sync>;

/// Returns an error for unconfigured mock methods.
pub(crate) fn not_configured<O>(method: &str) -> Result<O> {
    Err(Error::Api(ApiError {
        status: 501,
        kind: ErrorKind::NotImplemented,
        message: Some(format!("mock: {method} not configured")),
        ..Default::default()
    }))
}

/// Generates [`MockRuntime`] fields, `new()` defaults, and `on_*` builder methods
/// from a declarative handler table.
macro_rules! mock_handlers {
    ($(
        $(#[doc = $doc:expr])*
        $name:ident : $input:ty => $output:ty
    ),* $(,)?) => {
        /// Mock Nvisy Runtime API client for testing.
        ///
        /// Each method defaults to returning a `501 NotImplemented` error
        /// until configured with the corresponding `on_*` builder method.
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
                $(#[doc = $doc])*
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

    /// Sets the handler for [`InfraService::health`](crate::service::InfraService::health).
    on_health:        () => Health,
    /// Sets the handler for [`InfraService::analytics`](crate::service::InfraService::analytics).
    on_analytics:     () => AnalyticsSnapshot,
    /// Sets the handler for [`InfraService::openapi_spec`](crate::service::InfraService::openapi_spec).
    on_openapi_spec:  () => Value,

    // files

    /// Sets the handler for [`FileService::upload_file`](crate::service::FileService::upload_file).
    on_upload_file:   NewFile => FileId,
    /// Sets the handler for [`FileService::download_file`](crate::service::FileService::download_file).
    on_download_file: Uuid => File,
    /// Sets the handler for [`FileService::list_files`](crate::service::FileService::list_files).
    on_list_files:    Pagination => Page<FileEntry>,
    /// Sets the handler for [`FileService::delete_file`](crate::service::FileService::delete_file).
    on_delete_file:   Uuid => (),
    /// Sets the handler for [`FileService::delete_files`](crate::service::FileService::delete_files).
    on_delete_files:  () => (),

    // contexts

    /// Sets the handler for [`ContextService::upload_context`](crate::service::ContextService::upload_context).
    on_upload_context:   NewContext => ContextId,
    /// Sets the handler for [`ContextService::download_context`](crate::service::ContextService::download_context).
    on_download_context: Uuid => Context,
    /// Sets the handler for [`ContextService::list_contexts`](crate::service::ContextService::list_contexts).
    on_list_contexts:    Pagination => Page<ContextEntry>,
    /// Sets the handler for [`ContextService::delete_context`](crate::service::ContextService::delete_context).
    on_delete_context:   Uuid => (),
    /// Sets the handler for [`ContextService::delete_contexts`](crate::service::ContextService::delete_contexts).
    on_delete_contexts:  () => (),

    // runs

    /// Sets the handler for [`RunService::create_run`](crate::service::RunService::create_run).
    on_create_run:  NewRun => RunResult,
    /// Sets the handler for [`RunService::list_runs`](crate::service::RunService::list_runs).
    on_list_runs:   (RunQuery, Pagination) => Page<RunSummary>,
    /// Sets the handler for [`RunService::get_run`](crate::service::RunService::get_run).
    on_get_run:     Uuid => RunDetail,
    /// Sets the handler for [`RunService::cancel_run`](crate::service::RunService::cancel_run).
    on_cancel_run:  Uuid => (),
    /// Sets the handler for [`RunService::delete_run`](crate::service::RunService::delete_run).
    on_delete_run:  Uuid => (),
    /// Sets the handler for [`RunService::delete_runs`](crate::service::RunService::delete_runs).
    on_delete_runs: () => (),
}
