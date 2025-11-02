use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// npm 레지스트리 메타데이터 응답
#[derive(Debug, Deserialize)]
struct NpmRegistryResponse {
    #[serde(rename = "dist-tags")]
    dist_tags: DistTags,
    versions: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct DistTags {
    latest: String,
}

/// 패키지 버전 메타데이터
#[derive(Debug, Deserialize)]
struct PackageVersion {
    version: String,
    dist: Dist,
}

#[derive(Debug, Deserialize)]
struct Dist {
    tarball: String,
}

/// npm 패키지 리졸버
pub struct NpmResolver {
    cache_dir: PathBuf,
    registry_url: String,
}

impl NpmResolver {
    pub fn new() -> Result<Self> {
        // 캐시 디렉토리 설정 (OS별 기본 경로)
        let cache_dir = dirs::cache_dir()
            .context("캐시 디렉토리를 찾을 수 없습니다")?
            .join("executejs")
            .join("npm");

        // 캐시 디렉토리 생성
        fs::create_dir_all(&cache_dir).context("캐시 디렉토리를 생성할 수 없습니다")?;

        Ok(Self {
            cache_dir,
            registry_url: "https://registry.npmjs.org".to_string(),
        })
    }

    /// 캐시 디렉토리와 레지스트리 URL을 지정하여 생성
    pub fn with_cache_dir(cache_dir: PathBuf, registry_url: String) -> Result<Self> {
        // 캐시 디렉토리 생성
        fs::create_dir_all(&cache_dir).context("캐시 디렉토리를 생성할 수 없습니다")?;

        Ok(Self {
            cache_dir,
            registry_url,
        })
    }

    /// 캐시 디렉토리 경로 반환
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// 레지스트리 URL 반환
    pub fn registry_url(&self) -> &str {
        &self.registry_url
    }

    /// 패키지 다운로드 및 설치
    pub async fn install_package(
        &self,
        package_name: &str,
        version: Option<&str>,
    ) -> Result<PathBuf> {
        eprintln!(
            "[NpmResolver::install_package] 시작: package_name={}, version={:?}",
            package_name, version
        );

        // 패키지 버전 결정
        let version = match version {
            Some(v) => {
                eprintln!("[NpmResolver::install_package] 버전 지정됨: {}", v);
                v.to_string()
            }
            None => {
                eprintln!("[NpmResolver::install_package] 최신 버전 조회 중...");
                let latest = self.get_latest_version(package_name).await?;
                eprintln!("[NpmResolver::install_package] 최신 버전: {}", latest);
                latest
            }
        };

        // 캐시 경로
        let package_dir = self.cache_dir.join(package_name).join(&version);
        eprintln!(
            "[NpmResolver::install_package] 캐시 경로: {:?}",
            package_dir
        );

        // 이미 설치되어 있으면 스킵
        if package_dir.exists() {
            eprintln!("[NpmResolver::install_package] 캐시 디렉토리 존재 확인 중...");
            // package.json이 존재하는지 확인
            let package_json_path = package_dir.join("package").join("package.json");
            if package_json_path.exists() {
                eprintln!(
                    "[NpmResolver::install_package] 캐시된 패키지 사용: {:?}",
                    package_dir
                );
                return Ok(package_dir);
            }
            eprintln!("[NpmResolver::install_package] package.json 없음, 재다운로드 필요");
        }

        // 패키지 메타데이터 가져오기
        eprintln!("[NpmResolver::install_package] tarball URL 조회 중...");
        let tarball_url = self.get_tarball_url(package_name, &version).await?;
        eprintln!(
            "[NpmResolver::install_package] tarball URL: {}",
            tarball_url
        );

        // tarball 다운로드
        eprintln!("[NpmResolver::install_package] tarball 다운로드 시작...");
        let tarball_data = self.download_tarball(&tarball_url).await?;
        eprintln!(
            "[NpmResolver::install_package] tarball 다운로드 완료: {} bytes",
            tarball_data.len()
        );

        // 기존 디렉토리 삭제 후 재생성
        if package_dir.exists() {
            fs::remove_dir_all(&package_dir)
                .context("기존 패키지 디렉토리를 삭제할 수 없습니다")?;
        }
        fs::create_dir_all(&package_dir).context("패키지 디렉토리를 생성할 수 없습니다")?;

        // 압축 해제
        eprintln!("[NpmResolver::install_package] tarball 압축 해제 중...");
        self.extract_tarball(&tarball_data, &package_dir)?;
        eprintln!("[NpmResolver::install_package] 압축 해제 완료");

        eprintln!(
            "[NpmResolver::install_package] 패키지 설치 완료: {:?}",
            package_dir
        );
        Ok(package_dir)
    }

    /// 최신 버전 가져오기
    async fn get_latest_version(&self, package_name: &str) -> Result<String> {
        let url = format!("{}/{}", self.registry_url, package_name);
        let response = reqwest::get(&url).await?;
        let metadata: NpmRegistryResponse = response.json().await?;
        Ok(metadata.dist_tags.latest)
    }

    /// tarball URL 가져오기
    async fn get_tarball_url(&self, package_name: &str, version: &str) -> Result<String> {
        let url = format!("{}/{}", self.registry_url, package_name);
        let response = reqwest::get(&url).await?;
        let metadata: serde_json::Value = response.json().await?;

        let version_data = metadata["versions"][version]
            .as_object()
            .context("패키지 버전을 찾을 수 없습니다")?;

        let tarball_url = version_data["dist"]["tarball"]
            .as_str()
            .context("tarball URL을 찾을 수 없습니다")?;

        Ok(tarball_url.to_string())
    }

    /// tarball 다운로드
    async fn download_tarball(&self, url: &str) -> Result<Vec<u8>> {
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// tarball 압축 해제
    fn extract_tarball(&self, data: &[u8], target_dir: &Path) -> Result<()> {
        // 디렉토리 생성
        fs::create_dir_all(target_dir).context("타겟 디렉토리를 생성할 수 없습니다")?;

        // gzip 압축 해제
        let mut decoder = flate2::read::GzDecoder::new(data);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed)?;

        // tar 압축 해제
        let mut archive = tar::Archive::new(&decompressed[..]);
        archive.unpack(target_dir)?;

        Ok(())
    }

    /// 패키지의 진입점 파일 찾기 (ESM 우선: exports.import > module > main)
    pub fn find_entry_point(&self, package_dir: &Path) -> Result<PathBuf> {
        let package_json_path = package_dir.join("package").join("package.json");
        eprintln!(
            "[NpmResolver::find_entry_point] package.json 경로: {:?}",
            package_json_path
        );

        let package_json_content =
            fs::read_to_string(&package_json_path).context("package.json을 읽을 수 없습니다")?;
        eprintln!("[NpmResolver::find_entry_point] package.json 읽기 완료");

        let package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;
        eprintln!("[NpmResolver::find_entry_point] package.json 파싱 완료");

        // ESM 우선순위로 찾기: exports.import > module > exports.default > main
        let entry_point = package_json["exports"]
            .as_object()
            .and_then(|e| e.get("."))
            .and_then(|e| {
                e.as_object().and_then(|e| {
                    // exports["."].import.default 우선 확인 (ESM)
                    e.get("import").and_then(|i| {
                        i.as_object()
                            .and_then(|i| i.get("default"))
                            .and_then(|d| d.as_str())
                            .or_else(|| {
                                // exports["."].import가 직접 문자열인 경우
                                i.as_str()
                            })
                    })
                })
            })
            .or_else(|| package_json["module"].as_str()) // module 필드 (ESM)
            .or_else(|| {
                // exports["."]가 직접 문자열인 경우
                package_json["exports"]
                    .as_object()
                    .and_then(|e| e.get("."))
                    .and_then(|e| e.as_str())
            })
            .or_else(|| {
                // exports["."].require (CommonJS, 폴백)
                package_json["exports"]
                    .as_object()
                    .and_then(|e| e.get("."))
                    .and_then(|e| {
                        e.as_object().and_then(|e| e.get("require")).and_then(|r| {
                            r.as_object()
                                .and_then(|r| r.get("default"))
                                .and_then(|d| d.as_str())
                                .or_else(|| r.as_str())
                        })
                    })
            })
            .or_else(|| package_json["main"].as_str()) // main (CommonJS, 최종 폴백)
            .unwrap_or("index.js"); // 기본값

        eprintln!(
            "[NpmResolver::find_entry_point] 진입점 이름: {}",
            entry_point
        );

        let full_path = package_dir.join("package").join(entry_point);
        eprintln!("[NpmResolver::find_entry_point] 전체 경로: {:?}", full_path);
        eprintln!(
            "[NpmResolver::find_entry_point] 파일 존재 여부: {}",
            full_path.exists()
        );

        Ok(full_path)
    }

    /// 패키지의 타입 정의 파일 찾기 (package.json의 types 또는 typings 필드)
    pub fn find_type_definitions(&self, package_dir: &Path) -> Result<Option<PathBuf>> {
        let package_json_path = package_dir.join("package").join("package.json");
        eprintln!(
            "[NpmResolver::find_type_definitions] package.json 경로: {:?}",
            package_json_path
        );

        if !package_json_path.exists() {
            eprintln!("[NpmResolver::find_type_definitions] package.json 없음");
            return Ok(None);
        }

        let package_json_content =
            fs::read_to_string(&package_json_path).context("package.json을 읽을 수 없습니다")?;
        let package_json: serde_json::Value = serde_json::from_str(&package_json_content)?;

        // types 또는 typings 필드 확인
        let type_def = package_json["types"]
            .as_str()
            .or_else(|| package_json["typings"].as_str());

        if let Some(type_def_path) = type_def {
            eprintln!(
                "[NpmResolver::find_type_definitions] 타입 정의 경로 발견: {}",
                type_def_path
            );
            let full_path = package_dir.join("package").join(type_def_path);
            eprintln!(
                "[NpmResolver::find_type_definitions] 전체 경로: {:?}",
                full_path
            );
            eprintln!(
                "[NpmResolver::find_type_definitions] 파일 존재 여부: {}",
                full_path.exists()
            );

            if full_path.exists() {
                Ok(Some(full_path))
            } else {
                eprintln!("[NpmResolver::find_type_definitions] 타입 정의 파일이 존재하지 않음");
                Ok(None)
            }
        } else {
            eprintln!("[NpmResolver::find_type_definitions] types/typings 필드 없음");
            // @types/ 패키지 자동 검색 (예: lodash -> @types/lodash)
            let package_name = package_json["name"].as_str().unwrap_or("");

            // 스코프가 없는 패키지만 @types 검색
            if !package_name.starts_with('@') && !package_name.is_empty() {
                let types_package = format!("@types/{}", package_name);
                eprintln!(
                    "[NpmResolver::find_type_definitions] @types 패키지 검색 시도: {}",
                    types_package
                );
                // @types 패키지는 별도로 설치해야 하므로 여기서는 로그만 남김
                // 실제로는 npm: 프로토콜을 통해 로드 가능
            }

            Ok(None)
        }
    }
}
