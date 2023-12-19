use async_trait::async_trait;

use crate::error::Result;

#[async_trait]
pub trait Writer {
    fn open(&self) -> Result<()>;
    fn prepare(&self) -> Result<()>;
    fn close(&self) -> Result<()>;
    async fn start_write(&self) -> Result<()>;

    async fn start(&self) -> Result<()> {
        self.open()?;
        self.prepare()?;
        self.start_write().await?;
        self.close()?;
        Ok(())
    }
}

#[async_trait]
pub trait Reader {
    fn open(&self) -> Result<()>;
    fn prepare(&self) -> Result<()>;
    fn close(&self) -> Result<()>;
    async fn start_read(&self) -> Result<()>;

    async fn start(&self) -> Result<()> {
        self.open()?;
        self.prepare()?;
        self.start_read().await?;
        self.close()?;
        Ok(())
    }
}

pub trait Channel {
    fn open(&self) -> Result<()>;
    fn prepare(&self) -> Result<()>;
    fn close(&self) -> Result<()>;
}
