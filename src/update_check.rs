use reqwest::Client;
use semver::Version;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;
use tracing::{info, warn};

const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/Quesys-tech/vrcwatch.rs/releases/latest";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);
type CheckError = Box<dyn Error + Send + Sync>;

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
    draft: bool,
    prerelease: bool,
}

#[derive(Debug, PartialEq)]
enum UpdateCheckResult {
    UpdateAvailable {
        current: Version,
        latest: Version,
        url: String,
    },
    NoUpdate,
}

async fn fetch_latest_release(url: &str, timeout: Duration) -> Result<Release, reqwest::Error> {
    let client = Client::builder()
        .user_agent(concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION")
        ))
        .timeout(timeout)
        .build()?;

    client
        .get(url)
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await
}

fn compare_release(current: &str, release: Release) -> Result<UpdateCheckResult, CheckError> {
    let current =
        Version::parse(current).map_err(|e| format!("Invalid current version {current:?}: {e}"))?;
    if release.draft || release.prerelease {
        return Ok(UpdateCheckResult::NoUpdate);
    }
    let tag = release
        .tag_name
        .strip_prefix('v')
        .unwrap_or(&release.tag_name);
    let latest = Version::parse(tag)
        .map_err(|e| format!("Invalid release version {:?}: {e}", release.tag_name))?;

    // SemVer build metadata does not affect update precedence.
    if latest.pre.is_empty() && latest.cmp_precedence(&current).is_gt() {
        Ok(UpdateCheckResult::UpdateAvailable {
            current,
            latest,
            url: release.html_url,
        })
    } else {
        Ok(UpdateCheckResult::NoUpdate)
    }
}

fn notify_result(result: Result<UpdateCheckResult, CheckError>) {
    match result {
        Ok(UpdateCheckResult::UpdateAvailable {
            current,
            latest,
            url,
        }) => {
            info!(%current, %latest, %url, "A new VRCWatch release is available");
        }
        Ok(UpdateCheckResult::NoUpdate) => {}
        Err(error) => warn!(%error, "Failed to check for VRCWatch updates"),
    }
}

async fn check_and_notify(url: &str, timeout: Duration) {
    let result = match fetch_latest_release(url, timeout).await {
        Ok(release) => compare_release(env!("CARGO_PKG_VERSION"), release),
        Err(error) => Err(error.into()),
    };
    notify_result(result);
}

pub(crate) async fn check_for_updates() {
    check_and_notify(LATEST_RELEASE_URL, REQUEST_TIMEOUT).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, UdpSocket};
    use tokio::task::JoinHandle;
    use tracing::instrument::WithSubscriber;

    fn release(tag: &str) -> Release {
        Release {
            tag_name: tag.to_owned(),
            html_url: format!("https://github.com/Quesys-tech/vrcwatch.rs/releases/tag/{tag}"),
            draft: false,
            prerelease: false,
        }
    }

    #[test]
    fn compares_semver_versions() {
        for (current, tag, update) in [
            ("0.8.3", "v0.8.4", true),
            ("0.9.0", "v0.10.0", true),
            ("0.8.3", "0.8.4", true),
            ("0.8.3", "v0.8.3", false),
            ("0.9.0", "v0.8.3", false),
            ("0.8.3+local", "v0.8.3+release", false),
            ("0.8.4-rc.1", "v0.8.4", true),
        ] {
            let result = compare_release(current, release(tag)).unwrap();
            if update {
                assert_eq!(
                    result,
                    UpdateCheckResult::UpdateAvailable {
                        current: Version::parse(current).unwrap(),
                        latest: Version::parse(tag.strip_prefix('v').unwrap_or(tag)).unwrap(),
                        url: release(tag).html_url,
                    }
                );
            } else {
                assert_eq!(result, UpdateCheckResult::NoUpdate);
            }
        }
    }

    #[test]
    fn rejects_invalid_versions() {
        for (current, tag, message) in [
            ("invalid", "v0.8.4", "Invalid current version"),
            ("0.8.3", "invalid", "Invalid release version"),
            ("0.8.3", "vv0.8.4", "Invalid release version"),
        ] {
            assert!(compare_release(current, release(tag))
                .unwrap_err()
                .to_string()
                .contains(message));
        }
    }

    #[test]
    fn ignores_drafts_and_prereleases() {
        let mut draft = release("v0.8.4");
        draft.draft = true;
        let mut prerelease = release("v0.8.4");
        prerelease.prerelease = true;
        for release in [draft, prerelease, release("v0.8.4-rc.1")] {
            assert_eq!(
                compare_release("0.8.3", release).unwrap(),
                UpdateCheckResult::NoUpdate
            );
        }
    }

    // Each local mock accepts exactly one request; tests never contact GitHub.
    async fn mock_release(
        status: u16,
        body: &str,
        delay: Duration,
    ) -> (String, JoinHandle<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!(
            "http://{}/repos/Quesys-tech/vrcwatch.rs/releases/latest",
            listener.local_addr().unwrap()
        );
        let response = format!("HTTP/1.1 {status} Mock\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
        let task = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            let mut buffer = [0; 1024];
            while !request.ends_with(b"\r\n\r\n") {
                let count = socket.read(&mut buffer).await.unwrap();
                assert_ne!(count, 0);
                request.extend_from_slice(&buffer[..count]);
            }
            tokio::time::sleep(delay).await;
            let _ = socket.write_all(response.as_bytes()).await;
            String::from_utf8(request).unwrap().to_ascii_lowercase()
        });
        (url, task)
    }

    const RELEASE_JSON: &str = r#"{"tag_name":"v99.0.0","html_url":"https://github.com/Quesys-tech/vrcwatch.rs/releases/tag/v99.0.0","draft":false,"prerelease":false}"#;

    #[tokio::test]
    async fn fetches_release_with_github_headers() {
        let (url, server) = mock_release(200, RELEASE_JSON, Duration::ZERO).await;
        let release = fetch_latest_release(&url, REQUEST_TIMEOUT).await.unwrap();
        assert!(matches!(
            compare_release("0.8.3", release).unwrap(),
            UpdateCheckResult::UpdateAvailable { .. }
        ));
        let request = server.await.unwrap();
        assert!(
            request.starts_with("get /repos/quesys-tech/vrcwatch.rs/releases/latest http/1.1\r\n")
        );
        assert!(request.contains(concat!(
            "user-agent: vrcwatch-rs/",
            env!("CARGO_PKG_VERSION")
        )));
        assert!(request.contains("accept: application/vnd.github+json\r\n"));
        assert!(request.contains("x-github-api-version: 2022-11-28\r\n"));
    }

    #[derive(Clone, Default)]
    struct LogBuffer(Arc<Mutex<Vec<u8>>>);

    impl Write for LogBuffer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl LogBuffer {
        fn contents(&self) -> String {
            String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
        }
    }

    fn subscriber(buffer: &LogBuffer) -> impl tracing::Subscriber + Send + Sync + 'static {
        let buffer = buffer.clone();
        tracing_subscriber::fmt()
            .with_ansi(false)
            .without_time()
            .with_writer(move || buffer.clone())
            .finish()
    }

    #[tokio::test]
    async fn failures_warn_and_return_normally() {
        for (status, body) in [
            (403, RELEASE_JSON),
            (500, RELEASE_JSON),
            (200, "invalid json"),
            (200, "{}"),
            (
                200,
                r#"{"tag_name":"invalid","html_url":"https://example.com","draft":false,"prerelease":false}"#,
            ),
        ] {
            let (url, server) = mock_release(status, body, Duration::ZERO).await;
            let buffer = LogBuffer::default();
            check_and_notify(&url, REQUEST_TIMEOUT)
                .with_subscriber(subscriber(&buffer))
                .await;
            assert!(buffer.contents().contains("WARN"));
            assert!(buffer
                .contents()
                .contains("Failed to check for VRCWatch updates"));
            server.await.unwrap();
        }
    }

    #[tokio::test]
    async fn timeout_does_not_block_other_tasks() {
        let (url, server) = mock_release(200, RELEASE_JSON, Duration::from_secs(60)).await;
        let buffer = LogBuffer::default();
        let task_buffer = buffer.clone();
        let task = tokio::spawn(async move {
            check_and_notify(&url, Duration::from_secs(1))
                .with_subscriber(subscriber(&task_buffer))
                .await;
        });
        // Exercise the same OSC sender as the clock, during and after the request.
        tokio::task::yield_now().await;
        let receiver = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let sender = crate::osc_sender::OscSender::new(
            std::net::Ipv4Addr::LOCALHOST,
            0,
            std::net::Ipv4Addr::LOCALHOST,
            receiver.local_addr().unwrap().port(),
        );
        assert_osc_send(&sender, &receiver).await;
        assert!(!task.is_finished());
        tokio::time::timeout(Duration::from_secs(5), task)
            .await
            .unwrap()
            .unwrap();
        assert!(buffer.contents().contains("WARN"));
        assert!(buffer
            .contents()
            .contains("Failed to check for VRCWatch updates"));
        assert_osc_send(&sender, &receiver).await;
        server.abort();
        assert!(server.await.unwrap_err().is_cancelled());
    }

    async fn assert_osc_send(sender: &crate::osc_sender::OscSender, receiver: &UdpSocket) {
        sender
            .send(&0.5_f32, "/avatar/parameters/DateTimeSecondFA")
            .unwrap();
        let mut packet = [0; 256];
        let (count, _) =
            tokio::time::timeout(Duration::from_millis(500), receiver.recv_from(&mut packet))
                .await
                .unwrap()
                .unwrap();
        assert!(packet[..count].starts_with(b"/avatar/parameters/DateTimeSecondFA\0"));
    }

    #[tokio::test]
    async fn connection_failure_warns_and_returns_normally() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        drop(listener);
        let buffer = LogBuffer::default();
        check_and_notify(&url, REQUEST_TIMEOUT)
            .with_subscriber(subscriber(&buffer))
            .await;
        assert!(buffer.contents().contains("WARN"));
    }

    #[test]
    fn logs_update_details_and_is_quiet_without_update() {
        let buffer = LogBuffer::default();
        tracing::subscriber::with_default(subscriber(&buffer), || {
            notify_result(Ok(UpdateCheckResult::NoUpdate));
            assert!(buffer.contents().is_empty());
            notify_result(compare_release("0.8.3", release("v0.8.4")));
        });
        let log = buffer.contents();
        assert!(log.contains("current=0.8.3"));
        assert!(log.contains("latest=0.8.4"));
        assert!(log.contains("url=https://github.com/Quesys-tech/vrcwatch.rs/releases/tag/v0.8.4"));
    }
}
