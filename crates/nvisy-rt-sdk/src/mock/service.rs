//! Service trait implementations for [`MockRuntime`].

use serde_json::Value;
use uuid::Uuid;

use crate::error::Result;
use crate::model::{
    Analytics, Context, ContextId, File, FileId, Health, NewContext, NewFile, NewRun, Page,
    Pagination, RunDetail, RunResult, RunSummary,
};
use crate::service::RunQuery;

use super::MockRuntime;

impl crate::service::InfraService for MockRuntime {
    async fn health(&self) -> Result<Health> {
        (self.health)(())
    }

    async fn analytics(&self) -> Result<Analytics> {
        (self.analytics)(())
    }

    async fn openapi_spec(&self) -> Result<Value> {
        (self.openapi_spec)(())
    }
}

impl crate::service::FileService for MockRuntime {
    async fn upload_file(&self, request: &NewFile) -> Result<FileId> {
        (self.upload_file)(request.clone())
    }

    async fn download_file(&self, id: Uuid) -> Result<File> {
        (self.download_file)(id)
    }

    async fn list_files(&self, pagination: &Pagination) -> Result<Page<Uuid>> {
        (self.list_files)(pagination.clone())
    }

    #[cfg(feature = "stream")]
    fn list_files_stream(&self, _page_size: Option<u32>) -> crate::service::PageStream<Uuid> {
        unimplemented!("list_files_stream is not supported on MockRuntime")
    }

    async fn delete_file(&self, id: Uuid) -> Result<()> {
        (self.delete_file)(id)
    }

    async fn delete_files(&self) -> Result<()> {
        (self.delete_files)(())
    }
}

impl crate::service::ContextService for MockRuntime {
    async fn upload_context(&self, request: &NewContext) -> Result<ContextId> {
        (self.upload_context)(request.clone())
    }

    async fn download_context(&self, id: Uuid) -> Result<Context> {
        (self.download_context)(id)
    }

    async fn list_contexts(&self, pagination: &Pagination) -> Result<Page<Uuid>> {
        (self.list_contexts)(pagination.clone())
    }

    #[cfg(feature = "stream")]
    fn list_contexts_stream(&self, _page_size: Option<u32>) -> crate::service::PageStream<Uuid> {
        unimplemented!("list_contexts_stream is not supported on MockRuntime")
    }

    async fn delete_context(&self, id: Uuid) -> Result<()> {
        (self.delete_context)(id)
    }

    async fn delete_contexts(&self) -> Result<()> {
        (self.delete_contexts)(())
    }
}

impl crate::service::RunService for MockRuntime {
    async fn create_run(&self, request: &NewRun) -> Result<RunResult> {
        (self.create_run)(request.clone())
    }

    async fn list_runs(
        &self,
        query: &RunQuery,
        pagination: &Pagination,
    ) -> Result<Page<RunSummary>> {
        (self.list_runs)((query.clone(), pagination.clone()))
    }

    #[cfg(feature = "stream")]
    fn list_runs_stream(
        &self,
        _query: &RunQuery,
        _page_size: Option<u32>,
    ) -> crate::service::PageStream<RunSummary> {
        unimplemented!("list_runs_stream is not supported on MockRuntime")
    }

    async fn get_run(&self, id: Uuid) -> Result<RunDetail> {
        (self.get_run)(id)
    }

    async fn cancel_run(&self, id: Uuid) -> Result<()> {
        (self.cancel_run)(id)
    }

    async fn delete_run(&self, id: Uuid) -> Result<()> {
        (self.delete_run)(id)
    }

    async fn delete_runs(&self) -> Result<()> {
        (self.delete_runs)(())
    }
}
