//! Functionality for sending requests to Slack.

#[cfg(feature = "reqwest")]
mod reqwest_support {
    pub use self::reqwest::Error;
    use async_trait::async_trait;
    use reqwest_ as reqwest;
    use std::borrow::Borrow;

    use crate::async_impl::SlackWebRequestSender;

    type Client = reqwest::Client;

    #[async_trait]
    impl SlackWebRequestSender for Client {
        type Error = reqwest::Error;

        async fn get<I, K, V, S>(&self, method_url: S, params: I) -> Result<String, Self::Error>
        where
            I: IntoIterator + Send,
            K: AsRef<str>,
            V: AsRef<str>,
            I::Item: Borrow<(K, V)>,
            S: AsRef<str> + Send,
        {
            let mut url = reqwest::Url::parse(method_url.as_ref()).expect("Unable to parse url");

            url.query_pairs_mut().extend_pairs(params);

            Ok(self.get(url).send().await?.text().await?)
        }

        async fn post<I, K, V, S>(
            &self,
            method_url: S,
            form: &[(&str, &str)],
            headers: I,
        ) -> Result<String, Self::Error>
        where
            I: IntoIterator + Send,
            K: AsRef<str>,
            V: AsRef<str>,
            I::Item: Borrow<(K, V)>,
            S: AsRef<str> + Send,
        {
            let url = reqwest::Url::parse(method_url.as_ref()).expect("Unable to parse url");
            let mut req = self.post(url).form(form);
            for v in headers {
                let (k, v) = v.borrow();
                req = req.header(k.as_ref(), v.as_ref());
            }
            Ok(req.send().await?.text().await?)
        }
    }

    /// Provides a default `reqwest` client to give to the API functions to send requests.
    ///
    /// # Examples
    ///
    /// ```
    /// # let token = "some_token";
    /// let client = slack_api::requests::default_client().unwrap();
    /// let params = Default::default();
    /// let response = slack_api::channels::list(&client, &token, &params);
    /// ```
    pub fn default_client() -> Result<Client, reqwest::Error> {
        Ok(Client::new())
    }
}

#[cfg(feature = "reqwest")]
pub use self::reqwest_support::*;
