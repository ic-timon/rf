//! # cron
//!
//! cron 模块
//!
//! @author TimonQWQ
//! @date 2026-01-06

//! Cron job scheduler

use tokio_cron_scheduler::{Job, JobScheduler};

/// Cron scheduler wrapper
pub struct Cron {
    scheduler: JobScheduler,
}

impl Cron {
    /// Create a new cron scheduler
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let scheduler = JobScheduler::new().await?;
        Ok(Self { scheduler })
    }

    /// Add a cron job
    pub async fn add<F>(&self, schedule: &str, job: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let job = std::sync::Arc::new(job);
        let job_clone = job.clone();
        let job_async = Job::new_async(schedule, move |_uuid, _l| {
            let job = job_clone.clone();
            Box::pin(async move {
                job();
            })
        })?;
        self.scheduler.add(job_async).await?;
        Ok(())
    }

    /// Start the scheduler
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.scheduler.start().await?;
        Ok(())
    }
}
