use node_runtime::NpmManager;
use std::sync::Mutex;

// 테스트 간 격리를 위한 락
static TEST_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_parse_required_packages_require() {
    let _lock = TEST_LOCK.lock().unwrap();

    let code = r#"
        const lodash = require('lodash');
        const fs = require('fs');
    "#;

    let packages = NpmManager::parse_required_packages(code).unwrap();
    assert!(packages.contains(&"lodash".to_string()));
    // fs는 Node.js 내장 모듈이므로 npm 패키지 목록에 포함되지 않아야 함
    assert!(!packages.contains(&"fs".to_string()));
}

#[test]
fn test_parse_required_packages_import() {
    let _lock = TEST_LOCK.lock().unwrap();

    let code = r#"
        import { get } from 'lodash';
        import fs from 'fs';
    "#;

    let packages = NpmManager::parse_required_packages(code).unwrap();
    assert!(packages.contains(&"lodash".to_string()));
    // fs는 Node.js 내장 모듈이므로 npm 패키지 목록에 포함되지 않아야 함
    assert!(!packages.contains(&"fs".to_string()));
}

#[test]
fn test_parse_required_packages_dynamic_import() {
    let _lock = TEST_LOCK.lock().unwrap();

    let code = r#"
        const module = await import('lodash');
    "#;

    let packages = NpmManager::parse_required_packages(code).unwrap();
    assert!(packages.contains(&"lodash".to_string()));
}

#[test]
fn test_parse_required_packages_exclude_local() {
    let _lock = TEST_LOCK.lock().unwrap();

    let code = r#"
        const local = require('./local');
        const parent = require('../parent');
        const absolute = require('/absolute');
        const npm = require('lodash');
    "#;

    let packages = NpmManager::parse_required_packages(code).unwrap();
    assert!(!packages.contains(&"./local".to_string()));
    assert!(!packages.contains(&"../parent".to_string()));
    assert!(!packages.contains(&"/absolute".to_string()));
    assert!(packages.contains(&"lodash".to_string()));
}

#[test]
fn test_parse_required_packages_require_resolve() {
    let _lock = TEST_LOCK.lock().unwrap();

    let code = r#"
        const path = require.resolve('lodash');
    "#;

    let packages = NpmManager::parse_required_packages(code).unwrap();
    assert!(packages.contains(&"lodash".to_string()));
}

#[test]
fn test_parse_required_packages_empty() {
    let _lock = TEST_LOCK.lock().unwrap();

    let code = r#"
        console.log('Hello');
        const a = 5;
    "#;

    let packages = NpmManager::parse_required_packages(code).unwrap();
    assert!(packages.is_empty());
}

#[test]
fn test_parse_required_packages_scoped() {
    let _lock = TEST_LOCK.lock().unwrap();

    let code = r#"
        const pkg = require('@scope/package');
    "#;

    let packages = NpmManager::parse_required_packages(code).unwrap();
    assert!(packages.contains(&"@scope/package".to_string()));
}

#[test]
fn test_parse_required_packages_builtin_modules() {
    let _lock = TEST_LOCK.lock().unwrap();

    let code = r#"
        const fs = require('fs');
        const path = require('path');
        const http = require('http');
        const lodash = require('lodash');
    "#;

    let packages = NpmManager::parse_required_packages(code).unwrap();
    // 내장 모듈은 제외되어야 함
    assert!(!packages.contains(&"fs".to_string()));
    assert!(!packages.contains(&"path".to_string()));
    assert!(!packages.contains(&"http".to_string()));
    // npm 패키지만 포함되어야 함
    assert!(packages.contains(&"lodash".to_string()));
}
