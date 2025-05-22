pub mod messages;

pub struct NRFCloudClient {
    client: reqwest::Client,
    token: String,
    base_url: String,
}

impl NRFCloudClient {
    pub fn new(token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.to_string(),
            base_url: "https://api.nrfcloud.com/v1".to_string(),
        }
    }

    fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }

    /// Makes a GET request to the specified endpoint
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to call (will be appended to the base URL)
    ///
    /// # Returns
    ///
    /// A Result containing the response text or an error
    pub async fn get(&self, endpoint: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        response.error_for_status()?.text().await
    }

    /// Makes a GET request with query parameters to the specified endpoint
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to call (will be appended to the base URL)
    /// * `params` - Query parameters to include in the request
    ///
    /// # Returns
    ///
    /// A Result containing the response text or an error
    async fn get_with_params<T: serde::Serialize + ?Sized>(
        &self,
        endpoint: &str,
        params: &T,
    ) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .query(params)
            .send()
            .await?;

        response.error_for_status()?.text().await
    }

    /// Makes a GET request and deserializes the JSON response
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to call (will be appended to the base URL)
    ///
    /// # Returns
    ///
    /// A Result containing the deserialized JSON response or an error
    async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, reqwest::Error> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;

        response.error_for_status()?.json::<T>().await
    }

    /// Makes a GET request with query parameters and deserializes the JSON response
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint to call (will be appended to the base URL)
    /// * `params` - Query parameters to include in the request
    ///
    /// # Returns
    ///
    /// A Result containing the deserialized JSON response or an error
    async fn get_json_with_params<P: serde::Serialize + ?Sized, T: serde::de::DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &P,
    ) -> Result<T, reqwest::Error> {
        let url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .query(params)
            .send()
            .await?;

        response.error_for_status()?.json::<T>().await
    }
}
