use crate::{
    errors::Result,
    model::{ApplicationInformation, CreatePasteRequest, CreatedPaste, Metadata, Paste},
};
use reqwest::{Client, IntoUrl, Request, Url};
use serde::de::DeserializeOwned;

/// API client to perform unauthenticated requests to the
/// pasty API.
///
/// # Reference
/// Implementation according to the pasty API documentation:
/// https://github.com/lus/pasty/blob/master/API.md#api
#[derive(Clone)]
pub struct UnauthenticatedClient {
    client: Client,
    host: Url,
}

impl UnauthenticatedClient {
    /// Creates a new instance of UnauthenticatedClient with the given
    /// host URL.
    ///
    /// # Example
    /// ```
    /// # use pasty_rs::client::*;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = UnauthenticatedClient::new("https://pasty.lus.pm").unwrap();
    /// let res = client.application_information().await.unwrap();
    /// # }
    /// ```
    ///
    /// # Reference
    /// Implementation according to the pasty API documentation:
    /// https://github.com/lus/pasty/blob/master/API.md#api
    pub fn new(host: impl IntoUrl) -> Result<Self> {
        Ok(Self {
            client: Default::default(),
            host: host.into_url()?,
        })
    }

    /// Returns generall application information of the pasty instance.
    ///
    /// # Reference
    /// Binds to the `GET /api/v2/info` endpoint.
    /// https://github.com/lus/pasty/blob/master/API.md#unsecured-retrieve-application-information
    pub async fn application_information(&self) -> Result<ApplicationInformation> {
        let r = self.client.get(self.host.join("/api/v2/info")?).build()?;
        req_body(&self.client, r).await
    }

    /// Returns a pastes content by it's ID.
    ///
    /// # Reference
    /// Binds to the `GET /api/v2/pastes/{paste_id}` endpoint.
    /// https://github.com/lus/pasty/blob/master/API.md#unsecured-retrieve-a-paste
    pub async fn paste(&self, id: &str) -> Result<Paste> {
        let r = self
            .client
            .get(self.host.join(&format!("/api/v2/pastes/{id}"))?)
            .build()?;
        req_body(&self.client, r).await
    }

    /// Creates a paste with the given content and metadata.
    ///
    /// # Reference
    /// Binds to the `POST /api/v2/pastes` endpoint.
    /// https://github.com/lus/pasty/blob/master/API.md#unsecured-create-a-paste
    pub async fn create_paste(
        &self,
        content: impl Into<String>,
        metadata: Option<Metadata>,
    ) -> Result<CreatedPaste> {
        let r = self
            .client
            .post(self.host.join("/api/v2/pastes")?)
            .json(&CreatePasteRequest {
                content: content.into(),
                metadata,
            })
            .build()?;
        req_body(&self.client, r).await
    }

    /// Consumes the `UnauthenticatedClient` and a given paste modification or
    /// admin token to perform authenticated requests.
    pub fn authenticate(self, token: impl Into<String>) -> AuthenticatedClient {
        AuthenticatedClient {
            client: self,
            token: token.into(),
        }
    }
}

#[derive(Clone)]
pub struct AuthenticatedClient {
    client: UnauthenticatedClient,
    token: String,
}

/// API client to perform authenticated requests to the
/// pasty API.
///
/// This client can be created from an `UnauthenticatedClient` instance.
///
/// # Example
/// ```
/// # use pasty_rs::client::*;
/// # fn main() {
/// let client = UnauthenticatedClient::new("https://pasty.lus.pm").unwrap();
/// let auth_client = client.authenticate("some-token");
/// # }
/// ```
///
/// # Reference
/// Implementation according to the pasty API documentation:
/// https://github.com/lus/pasty/blob/master/API.md#api
impl AuthenticatedClient {
    /// Returns a reference to the inner `UnauthenticatedClient` instance.
    pub fn inner(&self) -> &UnauthenticatedClient {
        &self.client
    }

    /// Updates a given content and metadata by it's ID.
    ///
    /// # Reference
    /// Binds to the `PATCH /api/v2/pastes/{paste_id}` endpoint.
    /// https://github.com/lus/pasty/blob/master/API.md#paste_specific-update-a-paste
    pub async fn update_paste(
        &self,
        id: &str,
        content: impl Into<String>,
        metadata: Option<Metadata>,
    ) -> Result<()> {
        let r = self
            .client
            .client
            .patch(self.client.host.join(&format!("/api/v2/pastes/{id}"))?)
            .json(&CreatePasteRequest {
                content: content.into(),
                metadata,
            })
            .bearer_auth(&self.token)
            .build()?;
        req(&self.client.client, r).await
    }

    /// Deletes a paste by it's ID.
    ///
    /// # Reference
    /// Binds to the `DELETE /api/v2/pastes/{paste_id}` endpoint.
    /// https://github.com/lus/pasty/blob/master/API.md#paste_specific-delete-a-paste
    pub async fn delete_paste(&self, id: &str) -> Result<()> {
        let r = self
            .client
            .client
            .delete(self.client.host.join(&format!("/api/v2/pastes/{id}"))?)
            .bearer_auth(&self.token)
            .build()?;
        req(&self.client.client, r).await
    }
}

async fn req_body<T: DeserializeOwned>(client: &Client, req: Request) -> Result<T> {
    let res = client
        .execute(req)
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(res)
}

async fn req(client: &Client, req: Request) -> Result<()> {
    client.execute(req).await?.error_for_status()?;
    Ok(())
}
