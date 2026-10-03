use std::time::Duration;

use hudhook::tracing::info;
use pkg_version::*;
use semver::Version;

const UPDATE_URL: &str =
    "https://api.github.com/repos/veeenu/darksoulsiii-practice-tool/releases/latest";

pub enum Update {
    Available { url: String, notes: String },
    UpToDate,
    Error(String),
}

impl Update {
    pub fn check() -> Self {
        info!("Checking for updates...");
        Self::fetch().unwrap_or_else(Update::Error)
    }

    fn fetch() -> Result<Self, String> {
        #[derive(serde::Deserialize)]
        struct GithubRelease {
            tag_name: String,
            html_url: String,
            body: String,
        }

        let current_version = Version {
            major: pkg_version_major!(),
            minor: pkg_version_minor!(),
            patch: pkg_version_patch!(),
            pre: vec![],
            build: vec![],
        };

        // Fail fast when the network is unreachable instead of waiting for the
        // default timeout.
        let release = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(5))
            .build()
            .get(UPDATE_URL)
            .call()
            .map_err(|e| e.to_string())?
            .into_json::<GithubRelease>()
            .map_err(|e| e.to_string())?;

        let version = Version::parse(&release.tag_name).map_err(|e| e.to_string())?;

        if version <= current_version {
            return Ok(Update::UpToDate);
        }

        let notes = match release.body.find("## What's Changed") {
            Some(i) => release.body[..i].trim(),
            None => &release.body,
        };
        let notes = format!(
            "A new version of the practice tool is available!\n\nLatest version:    \
             {version}\nInstalled version: {current_version}\n\nRelease notes:\n{notes}\n",
        );

        Ok(Update::Available { url: release.html_url, notes })
    }
}
