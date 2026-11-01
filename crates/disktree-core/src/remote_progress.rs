//! One replaceable progress snapshot, never a queue of filesystem events.
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Stage {
    #[default]
    Connecting,
    CheckingHelper,
    UploadingHelper,
    Starting,
    Indexing,
    Finalizing,
}

impl Stage {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Connecting => "正在通过 SSH 连接",
            Self::CheckingHelper => "正在检查远程扫描器",
            Self::UploadingHelper => "正在上传远程扫描器",
            Self::Starting => "正在启动远程扫描器",
            Self::Indexing => "正在远程建立索引",
            Self::Finalizing => "正在生成树图",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Progress {
    pub stage: Stage,
    pub files: u64,
    pub dirs: u64,
    pub bytes: u64,
    pub errors: u64,
    pub directory: Option<PathBuf>,
    pub updated: Option<Instant>,
    pub preview: Option<Arc<crate::scan_preview::Preview>>,
}

/// Cap both wire heartbeats and UI publications at five per second.
pub(crate) const INTERVAL: Duration = Duration::from_millis(200);
pub(crate) type Shared = Arc<Mutex<Progress>>;

pub(crate) fn stage(shared: &Shared, value: Stage) {
    let mut progress = shared.lock().expect("remote progress lock");
    progress.stage = value;
    progress.updated = Some(Instant::now());
}
