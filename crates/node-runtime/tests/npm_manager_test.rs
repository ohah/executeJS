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
    assert!(packages.contains(&"fs".to_string()));
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
    assert!(packages.contains(&"fs".to_string()));
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
