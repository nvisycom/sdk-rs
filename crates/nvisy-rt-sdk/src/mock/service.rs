//! Service trait implementations for [`MockRuntime`].

use serde_json::Value;
use uuid::Uuid;

use super::MockRuntime;
use crate::error::Result;
use crate::model::{
    Analytics, Context, ContextId, File, FileId, Health, NewContext, NewFile, NewRun, Page,
    Pagination, RunDetail, RunResult, RunSummary,
};
use crate::service::{
    ContextService, FileService, InfraService, RunQuery, RunService,
};
#[cfg(feature = "stream")]
use crate::service::PageStream;

impl InfraService for MockRuntime {
    async fn health(&self) -> Result<Health> {
        (self.on_health)(())
    }

    async fn analytics(&self) -> Result<Analytics> {
        (self.on_analytics)(())
    }

    async fn openapi_spec(&self) -> Result<Value> {
        (self.on_openapi_spec)(())
    }
}

impl FileService for MockRuntime {
    async fn upload_file(&self, request: &NewFile) -> Result<FileId> {
        (self.on_upload_file)(request.clone())
    }

    async fn download_file(&self, id: Uuid) -> Result<File> {
        (self.on_download_file)(id)
    }

    async fn list_files(&self, pagination: &Pagination) -> Result<Page<Uuid>> {
        (self.on_list_files)(pagination.clone())
    }

    #[cfg(feature = "stream")]
    fn list_files_stream(&self, page_size: Option<u32>) -> PageStream<Uuid> {
        let handler = &self.on_list_files;
        let page = handler(Pagination::default()).unwrap_or(Page {
            total: 0,
            has_more: false,
            items: vec![],
        });
        let items = page.items;
        PageStream::new(
            Box::new(move |_| {
                let items = items.clone();
                Box::pin(async move {
                    Ok(Page {
                        total: items.len() as u32,
                        has_more: false,
                        items,
                    })
                })
            }),
            page_size.unwrap_or(100),
        )
    }

    async fn delete_file(&self, id: Uuid) -> Result<()> {
        (self.on_delete_file)(id)
    }

    async fn delete_files(&self) -> Result<()> {
        (self.on_delete_files)(())
    }
}

impl ContextService for MockRuntime {
    async fn upload_context(&self, request: &NewContext) -> Result<ContextId> {
        (self.on_upload_context)(request.clone())
    }

    async fn download_context(&self, id: Uuid) -> Result<Context> {
        (self.on_download_context)(id)
    }

    async fn list_contexts(&self, pagination: &Pagination) -> Result<Page<Uuid>> {
        (self.on_list_contexts)(pagination.clone())
    }

    #[cfg(feature = "stream")]
    fn list_contexts_stream(&self, page_size: Option<u32>) -> PageStream<Uuid> {
        let handler = &self.on_list_contexts;
        let page = handler(Pagination::default()).unwrap_or(Page {
            total: 0,
            has_more: false,
            items: vec![],
        });
        let items = page.items;
        PageStream::new(
            Box::new(move |_| {
                let items = items.clone();
                Box::pin(async move {
                    Ok(Page {
                        total: items.len() as u32,
                        has_more: false,
                        items,
                    })
                })
            }),
            page_size.unwrap_or(100),
        )
    }

    async fn delete_context(&self, id: Uuid) -> Result<()> {
        (self.on_delete_context)(id)
    }

    async fn delete_contexts(&self) -> Result<()> {
        (self.on_delete_contexts)(())
    }
}

impl RunService for MockRuntime {
    async fn create_run(&self, request: &NewRun) -> Result<RunResult> {
        (self.on_create_run)(request.clone())
    }

    async fn list_runs(
        &self,
        query: &RunQuery,
        pagination: &Pagination,
    ) -> Result<Page<RunSummary>> {
        (self.on_list_runs)((query.clone(), pagination.clone()))
    }

    #[cfg(feature = "stream")]
    fn list_runs_stream(
        &self,
        query: &RunQuery,
        page_size: Option<u32>,
    ) -> PageStream<RunSummary> {
        let handler = &self.on_list_runs;
        let page = handler((query.clone(), Pagination::default())).unwrap_or(Page {
            total: 0,
            has_more: false,
            items: vec![],
        });
        let items = page.items;
        PageStream::new(
            Box::new(move |_| {
                let items = items.clone();
                Box::pin(async move {
                    Ok(Page {
                        total: items.len() as u32,
                        has_more: false,
                        items,
                    })
                })
            }),
            page_size.unwrap_or(100),
        )
    }

    async fn get_run(&self, id: Uuid) -> Result<RunDetail> {
        (self.on_get_run)(id)
    }

    async fn cancel_run(&self, id: Uuid) -> Result<()> {
        (self.on_cancel_run)(id)
    }

    async fn delete_run(&self, id: Uuid) -> Result<()> {
        (self.on_delete_run)(id)
    }

    async fn delete_runs(&self) -> Result<()> {
        (self.on_delete_runs)(())
    }
}
