use crate::ResultExt;
use async_trait::async_trait;

#[async_trait]
pub trait ReqwestResponseExt: Sized {
    async fn shake(self, context: &str) -> crate::Result<Self>;
}

#[async_trait]
impl ReqwestResponseExt for reqwest::Response {
    async fn shake(self, context: &str) -> crate::Result<Self> {
        let is_error = self.error_for_status_ref().is_err();

        if is_error {
            let status_code = self.status();
            let err_msg = self
                .text()
                .await
                .context(format!("{}: Failed to read response's error message as text", context))?;

            return Err(crate::Error::new(format!(
                "{}: {}, Status: {}",
                context, err_msg, status_code
            )));
        }

        Ok(self)
    }
}
