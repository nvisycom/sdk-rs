//! Mock client definition and builder methods.

use serde_json::Value;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::model::{
    Analytics, ApiError, Context, ContextId, ErrorKind, File, FileId, Health, NewContext, NewFile,
    NewRun, Page, Pagination, RunDetail, RunResult, RunSummary,
};
use crate::service::RunQuery;

/// A boxed, thread-safe handler function.
pub(crate) type Handler<I, O> = Box<dyn Fn(I) -> Result<O> + Send + Sync>;

/// Default handler that returns an error for unconfigured methods.
pub(crate) fn not_configured<O>(method: &str) -> Result<O> {
    Err(Error::Api(ApiError {
        status: 501,
        kind: ErrorKind::NotImplemented,
        message: Some(format!("mock: {method} not configured")),
        ..Default::default()
    }))
}

/// A configurable mock client implementing all service traits.
///
/// Each method defaults to returning an error until configured
/// with the corresponding `on_*` builder method.
pub struct MockRuntime {
    // infra
    pub(crate) health: Handler<(), Health>,
    pub(crate) analytics: Handler<(), Analytics>,
    pub(crate) openapi_spec: Handler<(), Value>,

    // files
    pub(crate) upload_file: Handler<NewFile, FileId>,
    pub(crate) download_file: Handler<Uuid, File>,
    pub(crate) list_files: Handler<Pagination, Page<Uuid>>,
    pub(crate) delete_file: Handler<Uuid, ()>,
    pub(crate) delete_files: Handler<(), ()>,

    // contexts
    pub(crate) upload_context: Handler<NewContext, ContextId>,
    pub(crate) download_context: Handler<Uuid, Context>,
    pub(crate) list_contexts: Handler<Pagination, Page<Uuid>>,
    pub(crate) delete_context: Handler<Uuid, ()>,
    pub(crate) delete_contexts: Handler<(), ()>,

    // runs
    pub(crate) create_run: Handler<NewRun, RunResult>,
    pub(crate) list_runs: Handler<(RunQuery, Pagination), Page<RunSummary>>,
    pub(crate) get_run: Handler<Uuid, RunDetail>,
    pub(crate) cancel_run: Handler<Uuid, ()>,
    pub(crate) delete_run: Handler<Uuid, ()>,
    pub(crate) delete_runs: Handler<(), ()>,
}

impl MockRuntime {
    /// Creates a new mock with all methods returning "not configured" errors.
    pub fn new() -> Self {
        Self {
            health: Box::new(|_| not_configured("health")),
            analytics: Box::new(|_| not_configured("analytics")),
            openapi_spec: Box::new(|_| not_configured("openapi_spec")),

            upload_file: Box::new(|_| not_configured("upload_file")),
            download_file: Box::new(|_| not_configured("download_file")),
            list_files: Box::new(|_| not_configured("list_files")),
            delete_file: Box::new(|_| not_configured("delete_file")),
            delete_files: Box::new(|_| not_configured("delete_files")),

            upload_context: Box::new(|_| not_configured("upload_context")),
            download_context: Box::new(|_| not_configured("download_context")),
            list_contexts: Box::new(|_| not_configured("list_contexts")),
            delete_context: Box::new(|_| not_configured("delete_context")),
            delete_contexts: Box::new(|_| not_configured("delete_contexts")),

            create_run: Box::new(|_| not_configured("create_run")),
            list_runs: Box::new(|_| not_configured("list_runs")),
            get_run: Box::new(|_| not_configured("get_run")),
            cancel_run: Box::new(|_| not_configured("cancel_run")),
            delete_run: Box::new(|_| not_configured("delete_run")),
            delete_runs: Box::new(|_| not_configured("delete_runs")),
        }
    }

    // infra builders

    pub fn on_health(mut self, f: impl Fn(()) -> Result<Health> + Send + Sync + 'static) -> Self {
        self.health = Box::new(f);
        self
    }

    pub fn on_analytics(
        mut self,
        f: impl Fn(()) -> Result<Analytics> + Send + Sync + 'static,
    ) -> Self {
        self.analytics = Box::new(f);
        self
    }

    pub fn on_openapi_spec(
        mut self,
        f: impl Fn(()) -> Result<Value> + Send + Sync + 'static,
    ) -> Self {
        self.openapi_spec = Box::new(f);
        self
    }

    // file builders

    pub fn on_upload_file(
        mut self,
        f: impl Fn(NewFile) -> Result<FileId> + Send + Sync + 'static,
    ) -> Self {
        self.upload_file = Box::new(f);
        self
    }

    pub fn on_download_file(
        mut self,
        f: impl Fn(Uuid) -> Result<File> + Send + Sync + 'static,
    ) -> Self {
        self.download_file = Box::new(f);
        self
    }

    pub fn on_list_files(
        mut self,
        f: impl Fn(Pagination) -> Result<Page<Uuid>> + Send + Sync + 'static,
    ) -> Self {
        self.list_files = Box::new(f);
        self
    }

    pub fn on_delete_file(
        mut self,
        f: impl Fn(Uuid) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        self.delete_file = Box::new(f);
        self
    }

    pub fn on_delete_files(mut self, f: impl Fn(()) -> Result<()> + Send + Sync + 'static) -> Self {
        self.delete_files = Box::new(f);
        self
    }

    // context builders

    pub fn on_upload_context(
        mut self,
        f: impl Fn(NewContext) -> Result<ContextId> + Send + Sync + 'static,
    ) -> Self {
        self.upload_context = Box::new(f);
        self
    }

    pub fn on_download_context(
        mut self,
        f: impl Fn(Uuid) -> Result<Context> + Send + Sync + 'static,
    ) -> Self {
        self.download_context = Box::new(f);
        self
    }

    pub fn on_list_contexts(
        mut self,
        f: impl Fn(Pagination) -> Result<Page<Uuid>> + Send + Sync + 'static,
    ) -> Self {
        self.list_contexts = Box::new(f);
        self
    }

    pub fn on_delete_context(
        mut self,
        f: impl Fn(Uuid) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        self.delete_context = Box::new(f);
        self
    }

    pub fn on_delete_contexts(
        mut self,
        f: impl Fn(()) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        self.delete_contexts = Box::new(f);
        self
    }

    // run builders

    pub fn on_create_run(
        mut self,
        f: impl Fn(NewRun) -> Result<RunResult> + Send + Sync + 'static,
    ) -> Self {
        self.create_run = Box::new(f);
        self
    }

    pub fn on_list_runs(
        mut self,
        f: impl Fn((RunQuery, Pagination)) -> Result<Page<RunSummary>> + Send + Sync + 'static,
    ) -> Self {
        self.list_runs = Box::new(f);
        self
    }

    pub fn on_get_run(
        mut self,
        f: impl Fn(Uuid) -> Result<RunDetail> + Send + Sync + 'static,
    ) -> Self {
        self.get_run = Box::new(f);
        self
    }

    pub fn on_cancel_run(mut self, f: impl Fn(Uuid) -> Result<()> + Send + Sync + 'static) -> Self {
        self.cancel_run = Box::new(f);
        self
    }

    pub fn on_delete_run(mut self, f: impl Fn(Uuid) -> Result<()> + Send + Sync + 'static) -> Self {
        self.delete_run = Box::new(f);
        self
    }

    pub fn on_delete_runs(mut self, f: impl Fn(()) -> Result<()> + Send + Sync + 'static) -> Self {
        self.delete_runs = Box::new(f);
        self
    }
}

impl Default for MockRuntime {
    fn default() -> Self {
        Self::new()
    }
}
