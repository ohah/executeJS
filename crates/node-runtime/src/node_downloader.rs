use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

pub const NODE_VERSION: &str = "v24.12.0";

fn base_url() -> String {
    format!("https://nodejs.org/dist/{}/", NODE_VERSION)
}

pub struct NodeDownloader;

impl NodeDownloader {
    /// 캐시 디렉토리 경로 반환
    pub fn cache_dir() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .context("캐시 디렉토리를 찾을 수 없습니다")?
            .join("executejs")
            .join("node-runtime");

        fs::create_dir_all(&cache_dir).context("캐시 디렉토리를 생성할 수 없습니다")?;

        Ok(cache_dir)
    }

    /// OS 및 아키텍처 정보 반환
    pub fn get_platform_info() -> Result<(String, String, String, String)> {
        let (os_name, arch, extension, binary_name) = if cfg!(target_os = "windows") {
            let arch = if cfg!(target_arch = "aarch64") {
                "arm64"
            } else {
                "x64"
            };
            ("win", arch, "zip", "node.exe")
        } else if cfg!(target_os = "macos") {
            let arch = if cfg!(target_arch = "aarch64") {
                "arm64"
            } else {
                "x64"
            };
            ("darwin", arch, "tar.xz", "node")
        } else if cfg!(target_os = "linux") {
            let arch = if cfg!(target_arch = "aarch64") {
                "arm64"
            } else {
                "x64"
            };
            ("linux", arch, "tar.xz", "node")
        } else {
            anyhow::bail!("지원하지 않는 운영체제입니다: {}", std::env::consts::OS);
        };

        Ok((
            os_name.to_string(),
            arch.to_string(),
            extension.to_string(),
            binary_name.to_string(),
        ))
    }

    /// Node.js 바이너리 경로 반환 (다운로드 필요 시 다운로드)
    pub async fn ensure_node_binary() -> Result<PathBuf> {
        let (os_name, arch, _extension, binary_name) = Self::get_platform_info()?;
        let cache_dir = Self::cache_dir()?;
        // find_node_binary와 동일한 경로 형식 사용
        let node_dir = cache_dir.join(format!("node-{}-{}-{}", NODE_VERSION, os_name, arch));
        let node_path = node_dir.join(&binary_name);

        // 이미 존재하면 반환
        if node_path.exists() {
            tracing::info!("Node.js 바이너리 발견: {}", node_path.display());
            Self::set_permissions_if_needed(&node_path)?;
            return Ok(node_path);
        }

        // 다운로드 필요
        tracing::info!(
            "Node.js 바이너리 다운로드 시작... (경로: {})",
            node_path.display()
        );
        if let Err(e) = Self::download_node_binary().await {
            tracing::error!("Node.js 바이너리 다운로드 실패: {}", e);
            return Err(e);
        }

        // 다운로드 후 다시 확인
        if node_path.exists() {
            Self::set_permissions_if_needed(&node_path)?;
            Ok(node_path)
        } else {
            anyhow::bail!(
                "Node.js 바이너리 다운로드 후에도 파일을 찾을 수 없습니다: {}",
                node_path.display()
            );
        }
    }

    /// Node.js 바이너리 다운로드
    async fn download_node_binary() -> Result<()> {
        let (os_name, arch, extension, binary_name) = Self::get_platform_info()?;
        let cache_dir = Self::cache_dir()?;
        // find_node_binary와 동일한 경로 형식 사용
        let node_dir = cache_dir.join(format!("node-{}-{}-{}", NODE_VERSION, os_name, arch));

        let file_name = format!("node-{}-{}-{}.{}", NODE_VERSION, os_name, arch, extension);
        let download_url = format!("{}{}", base_url(), file_name);
        let lock_file = cache_dir.join(format!("{}.lock", file_name));

        // 동시 다운로드 보호: 락 파일 생성
        let _lock_guard = Self::acquire_download_lock(&lock_file).await?;

        tracing::info!("Node.js 다운로드 시작: {}", download_url);
        tracing::info!("캐시 디렉토리: {}", cache_dir.display());
        tracing::info!("타겟 디렉토리: {}", node_dir.display());

        // 락 획득 후 다시 확인 (다른 프로세스가 이미 다운로드 완료했을 수 있음)
        let node_path = node_dir.join(&binary_name);
        if node_path.exists() {
            tracing::info!("다른 프로세스가 이미 다운로드를 완료했습니다");
            // 락 파일 정리
            let _ = fs::remove_file(&lock_file);
            return Ok(());
        }

        // 다운로드
        let response = reqwest::get(&download_url)
            .await
            .context("Node.js 다운로드 실패")?;

        // HTTP 응답 상태 코드 확인
        if !response.status().is_success() {
            anyhow::bail!(
                "Node.js 다운로드 실패: HTTP {} - {}",
                response.status(),
                response
                    .status()
                    .canonical_reason()
                    .unwrap_or("알 수 없는 오류")
            );
        }

        let bytes = response
            .bytes()
            .await
            .context("다운로드 데이터 읽기 실패")?;

        // SHA256 체크섬 검증
        tracing::info!("파일 무결성 검증 중...");
        Self::verify_checksum(&file_name, &bytes).await?;
        tracing::info!("파일 무결성 검증 완료");

        // 임시 파일에 저장
        let temp_file = cache_dir.join(&file_name);
        let mut file = fs::File::create(&temp_file).context("임시 파일 생성 실패")?;
        file.write_all(&bytes).context("파일 쓰기 실패")?;
        drop(file); // 파일 핸들 닫기

        // 압축 해제
        tracing::info!("압축 해제 중...");
        fs::create_dir_all(&cache_dir).context("캐시 디렉토리 생성 실패")?;

        // 압축 해제 후 디렉토리 이름 (Node.js 배포본의 실제 디렉토리 이름)
        let extracted_dir_name = format!("node-{}-{}-{}", NODE_VERSION, os_name, arch);
        let extracted_dir = cache_dir.join(&extracted_dir_name);

        if extension == "tar.xz" {
            // tar.xz 압축 해제
            let tar_xz = fs::File::open(&temp_file)?;
            let tar = XzDecoder::new(tar_xz);
            let mut archive = Archive::new(tar);
            archive
                .unpack(&cache_dir)
                .context("tar.xz 압축 해제 실패")?;
        } else if extension == "zip" {
            // Windows: zip 압축 해제
            let zip_file = fs::File::open(&temp_file)?;
            let mut archive = ZipArchive::new(zip_file)?;
            archive.extract(&cache_dir).context("zip 압축 해제 실패")?;
        } else {
            anyhow::bail!("지원하지 않는 압축 형식: {}", extension);
        }

        // 바이너리 찾기 및 이동
        let source_binary = if os_name == "win" {
            extracted_dir.join(&binary_name)
        } else {
            extracted_dir.join("bin").join(&binary_name)
        };

        if !source_binary.exists() {
            anyhow::bail!(
                "압축 해제 후 바이너리를 찾을 수 없습니다: {}",
                source_binary.display()
            );
        }

        // 타겟 디렉토리 생성
        fs::create_dir_all(&node_dir).context("Node.js 디렉토리 생성 실패")?;

        // 바이너리 복사
        let target_binary = node_dir.join(&binary_name);
        tracing::info!("소스 바이너리: {}", source_binary.display());
        tracing::info!("타겟 바이너리: {}", target_binary.display());

        if source_binary != target_binary {
            tracing::info!("바이너리 복사 중...");
            fs::copy(&source_binary, &target_binary).context("바이너리 복사 실패")?;
            tracing::info!("바이너리 복사 완료");
        } else {
            tracing::info!("바이너리가 이미 올바른 위치에 있습니다");
        }

        // 복사 후 확인
        if !target_binary.exists() {
            anyhow::bail!(
                "바이너리 복사 후에도 파일을 찾을 수 없습니다: {}",
                target_binary.display()
            );
        }

        // 임시 파일 정리
        tracing::info!("임시 파일 정리 중...");
        if let Err(e) = fs::remove_file(&temp_file) {
            tracing::warn!("임시 파일 삭제 실패 ({}): {}", temp_file.display(), e);
        }

        // extracted_dir와 node_dir가 같은 경우 삭제하지 않음 (바이너리가 이미 올바른 위치에 있음)
        if extracted_dir != node_dir && extracted_dir.exists() {
            tracing::info!("압축 해제 디렉토리 정리 중: {}", extracted_dir.display());
            if let Err(e) = fs::remove_dir_all(&extracted_dir) {
                tracing::warn!(
                    "압축 해제 디렉토리 삭제 실패 ({}): {}",
                    extracted_dir.display(),
                    e
                );
            }
        } else {
            tracing::info!("압축 해제 디렉토리가 타겟 디렉토리와 동일하므로 정리하지 않음");
        }

        tracing::info!(
            "Node.js 바이너리 다운로드 완료: {}",
            target_binary.display()
        );

        // 락 파일 정리
        if let Err(e) = fs::remove_file(&lock_file) {
            tracing::warn!("락 파일 삭제 실패 ({}): {}", lock_file.display(), e);
        }

        Ok(())
    }

    /// 실행 권한 설정
    fn set_permissions_if_needed(node_path: &Path) -> Result<()> {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(node_path)?;
            let perms = metadata.permissions();
            if perms.mode() & 0o111 == 0 {
                let mut new_perms = perms.clone();
                new_perms.set_mode(0o755);
                fs::set_permissions(node_path, new_perms)?;
            }
        }
        Ok(())
    }

    /// 다운로드 락 획득 (동시 다운로드 방지)
    /// 반환된 파일 핸들은 함수가 끝날 때까지 유지되어 락이 유지됩니다.
    async fn acquire_download_lock(lock_file: &Path) -> Result<fs::File> {
        use fs2::FileExt;
        use std::fs::OpenOptions;

        // 락 파일 생성 및 배타적 락 획득
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(lock_file)
            .context("락 파일 생성 실패")?;

        // 비동기적으로 락 획득 시도 (블로킹)
        let file_clone = file.try_clone()?;
        tokio::task::spawn_blocking(move || {
            file_clone
                .lock_exclusive()
                .context("다운로드 락 획득 실패 (다른 프로세스가 다운로드 중일 수 있음)")
        })
        .await
        .context("락 획득 작업 실패")??;

        Ok(file)
    }

    /// SHA256 체크섬 검증
    async fn verify_checksum(file_name: &str, file_bytes: &[u8]) -> Result<()> {
        // SHASUMS256.txt 다운로드
        let checksums_url = format!("{}SHASUMS256.txt", base_url());
        tracing::debug!("체크섬 파일 다운로드: {}", checksums_url);

        let response = reqwest::get(&checksums_url)
            .await
            .context("체크섬 파일 다운로드 실패")?;

        if !response.status().is_success() {
            tracing::warn!(
                "체크섬 파일 다운로드 실패: HTTP {} - 무결성 검증을 건너뜁니다",
                response.status()
            );
            return Ok(()); // 체크섬 검증 실패 시 경고만 하고 계속 진행
        }

        let checksums_text = response.text().await.context("체크섬 파일 읽기 실패")?;

        // 파일의 SHA256 계산
        let mut hasher = Sha256::new();
        hasher.update(file_bytes);
        let computed_hash = hasher.finalize();
        let computed_hash_hex = format!("{:x}", computed_hash);

        // 체크섬 파일에서 해당 파일의 해시 찾기
        for line in checksums_text.lines() {
            if line.ends_with(file_name) {
                let expected_hash = line
                    .split_whitespace()
                    .next()
                    .context("체크섬 파일 형식 오류")?;

                if computed_hash_hex == expected_hash {
                    tracing::info!("파일 무결성 검증 성공");
                    return Ok(());
                } else {
                    anyhow::bail!(
                        "파일 무결성 검증 실패: 예상 해시 {}, 실제 해시 {}",
                        expected_hash,
                        computed_hash_hex
                    );
                }
            }
        }

        tracing::warn!(
            "체크섬 파일에서 {}에 대한 해시를 찾을 수 없습니다 - 무결성 검증을 건너뜁니다",
            file_name
        );
        Ok(()) // 체크섬 파일에 없으면 경고만 하고 계속 진행
    }
}
