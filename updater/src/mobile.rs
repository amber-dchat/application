use semver::Version;
use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle}, AppHandle, Runtime
};
use crate::models::*;
use std::sync::LazyLock;



use reqwest::{Client, ClientBuilder};
pub(crate) static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  ClientBuilder::new()
    .user_agent("Amber DChat Android")
    .build()
    .unwrap()
});


// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Updater<R>> {
  let handle = api.register_android_plugin("com.plugin.aupdater", "Updater")?;
  Ok(Updater(app.clone(), handle))
}

/// Access to the updater APIs.
pub struct Updater<R: Runtime>(AppHandle<R>, PluginHandle<R>);

impl<R: Runtime> Updater<R> {
  pub async fn get_release(&self) -> crate::Result<Release> {
    let response: Release = CLIENT
      .get("https://api.github.com/repos/amber-dchat/application/releases/latest")
      .send()
      .await?
      .json()
      .await?;

    Ok(response)
  }

  pub async fn check(&self) -> crate::Result<Option<Update<R>>> {
    let Release { assets, tag_name } = self.get_release().await?;

    let new = Version::parse(&tag_name).unwrap_or(Version::new(0, 0, 0));

    let current = &self.0.package_info().version;

    Ok(
      if &new > current {
        let Some(Asset { browser_download_url, .. }) = assets.into_iter().find(|x| x.name == "app-universal.apk") else {
          return Ok(None);
        };

        Some(Update { download: browser_download_url, handle: self.1.clone() })
      } else {
        None
      }
    )
  }
}
