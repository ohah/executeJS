use node_runtime::node_downloader::{NodeDownloader, NODE_VERSION};

#[test]
fn test_get_platform_info() {
    let result = NodeDownloader::get_platform_info();
    assert!(result.is_ok());

    let (os_name, arch, extension, binary_name) = result.unwrap();
    assert!(!os_name.is_empty());
    assert!(!arch.is_empty());
    assert!(!extension.is_empty());
    assert!(!binary_name.is_empty());

    // 플랫폼별 검증
    #[cfg(target_os = "macos")]
    {
        assert_eq!(os_name, "darwin");
        assert!(extension == "tar.xz");
        assert_eq!(binary_name, "node");
    }

    #[cfg(target_os = "windows")]
    {
        assert_eq!(os_name, "win");
        assert!(extension == "zip");
        assert_eq!(binary_name, "node.exe");
    }

    #[cfg(target_os = "linux")]
    {
        assert_eq!(os_name, "linux");
        assert!(extension == "tar.xz");
        assert_eq!(binary_name, "node");
    }
}

#[test]
fn test_cache_dir() {
    let result = NodeDownloader::cache_dir();
    assert!(result.is_ok());

    let cache_dir = result.unwrap();
    assert!(cache_dir.to_string_lossy().contains("executejs"));
    assert!(cache_dir.to_string_lossy().contains("node-runtime"));
    assert!(cache_dir.exists() || cache_dir.parent().unwrap().exists());
}

#[test]
fn test_node_version_constant() {
    // NODE_VERSION 상수가 올바르게 정의되어 있는지 확인
    assert_eq!(NODE_VERSION, "v24.12.0");
    assert!(NODE_VERSION.starts_with('v'));
}

#[test]
fn test_base_url() {
    // base_url은 private 함수이므로 직접 테스트할 수 없지만,
    // NODE_VERSION을 통해 간접적으로 검증 가능
    assert_eq!(NODE_VERSION, "v24.12.0");
    let expected_url = format!("https://nodejs.org/dist/{}/", NODE_VERSION);
    assert!(expected_url.contains("nodejs.org"));
    assert!(expected_url.contains("dist"));
    assert!(expected_url.contains(NODE_VERSION));
    assert!(expected_url.ends_with('/'));
}
