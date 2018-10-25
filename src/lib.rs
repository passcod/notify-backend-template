extern crate notify_backend as backend;

use backend::prelude::*;
use backend::Buffer;
use std::sync::Arc;

// TODO: explain this has to have this name??
const BACKEND_NAME: &str = "{{project-name}}";

#[derive(Debug)]
pub struct Backend {
    buffer: Buffer,
    reg: Arc<MioRegistration>,
    watches: Vec<PathBuf>,
}

impl NotifyBackend for Backend {
    fn name() -> &'static str {
        // TODO: explain why this
        BACKEND_NAME
    }

    fn new(watches: Vec<PathBuf>) -> NewBackendResult {
        // TODO: explaining comments
        let (reg, readiness) = MioRegistration::new2();
        readiness.set_readiness(MioReady::readable());

        Ok(Box::new(Self {
            buffer: Buffer::default(),
            reg: Arc::new(reg),
            watches,
        }))
    }

    fn capabilities() -> Vec<Capability> {
        // Remove those that are not supported by your chosen platform/project.
        // See https://docs.rs/notify/TODO for a description of each.
        vec![
            Capability::EmitOnAccess,
            Capability::FollowSymlinks,
            Capability::TrackRelated,
            Capability::WatchFiles,
            Capability::WatchFolders,
            Capability::WatchNewFolders,
            Capability::WatchRecursively,
        ]
    }

    fn driver(&self) -> Box<Evented> {
        // TODO: explain/ref drivers
        Box::new(self.reg.clone())
    }
}

impl Stream for Backend {
    type Item = StreamItem;
    type Error = StreamError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        // TODO: explain what this should do
        self.buffer.poll()
    }
}

// TODO: explain this is required
impl Drop for Backend {
    fn drop(&mut self) {}
}
