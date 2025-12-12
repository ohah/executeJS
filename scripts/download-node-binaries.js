#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const { createWriteStream } = require('fs');

const NODE_VERSION = 'v24.12.0';
const BASE_URL = `https://nodejs.org/dist/${NODE_VERSION}/`;

// OS 및 아키텍처 매핑
const getPlatformInfo = () => {
  const platform = process.platform;
  const arch = process.arch;

  if (platform === 'darwin') {
    return {
      os: 'darwin',
      arch: arch === 'arm64' ? 'arm64' : 'x64',
      extension: 'tar.xz', // 더 작은 파일 크기
      binaryName: 'node',
    };
  } else if (platform === 'win32') {
    return {
      os: 'win',
      arch: arch === 'arm64' ? 'arm64' : 'x64',
      extension: 'zip',
      binaryName: 'node.exe',
    };
  } else {
    throw new Error(`Unsupported platform: ${platform}. Only macOS and Windows are supported.`);
  }
};

// 파일 다운로드
const downloadFile = async (url, destPath) => {
  return new Promise((resolve, reject) => {
    console.log(`다운로드 중: ${url}`);
    const file = createWriteStream(destPath);

    https
      .get(url, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          // 리다이렉트 처리
          return downloadFile(response.headers.location, destPath).then(resolve).catch(reject);
        }

        if (response.statusCode !== 200) {
          reject(new Error(`다운로드 실패: ${response.statusCode}`));
          return;
        }

        const totalSize = parseInt(response.headers['content-length'], 10);
        let downloadedSize = 0;

        response.on('data', (chunk) => {
          downloadedSize += chunk.length;
          const percent = totalSize ? ((downloadedSize / totalSize) * 100).toFixed(1) : '0.0';
          const downloadedMB = (downloadedSize / 1024 / 1024).toFixed(2);
          const totalMB = totalSize ? (totalSize / 1024 / 1024).toFixed(2) : '?';
          process.stdout.write(`\r진행률: ${percent}% (${downloadedMB} MB / ${totalMB} MB)`);
        });

        response.pipe(file);

        file.on('finish', () => {
          file.close();
          console.log('\n다운로드 완료!');
          resolve();
        });

        file.on('error', (err) => {
          fs.unlink(destPath, () => {});
          reject(err);
        });
      })
      .on('error', (err) => {
        reject(err);
      });
  });
};

// 압축 해제
const extractArchive = async (archivePath, extractDir) => {
  const ext = path.extname(archivePath);
  const platform = process.platform;

  console.log(`압축 해제 중: ${archivePath}`);

  if (ext === '.zip') {
    // Windows: unzip 사용
    try {
      execSync(`unzip -q "${archivePath}" -d "${extractDir}"`, { stdio: 'inherit' });
    } catch (error) {
      // unzip이 없으면 PowerShell 사용
      execSync(`powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${extractDir}' -Force"`, {
        stdio: 'inherit',
      });
    }
  } else if (ext === '.xz' || archivePath.endsWith('.tar.xz')) {
    // tar.xz 압축 해제
    execSync(`tar -xJf "${archivePath}" -C "${extractDir}"`, { stdio: 'inherit' });
  } else if (ext === '.gz' || archivePath.endsWith('.tar.gz')) {
    // tar.gz 압축 해제
    execSync(`tar -xzf "${archivePath}" -C "${extractDir}"`, { stdio: 'inherit' });
  }

  console.log('압축 해제 완료!');
};

// 바이너리 복사
const copyBinary = async (platformInfo, extractDir) => {
  const { os, arch, binaryName } = platformInfo;
  const nodeDir = `node-${NODE_VERSION}-${os}-${arch}`;
  // Windows는 루트에, macOS/Linux는 bin/ 디렉토리에 있습니다
  const sourcePath =
    os === 'win' ? path.join(extractDir, nodeDir, binaryName) : path.join(extractDir, nodeDir, 'bin', binaryName);
  // src-tauri/resources/에만 복사 (개발/빌드 모두 동일 경로 사용)
  const targetDirs = [
    path.join(__dirname, '..', 'apps', 'executeJS', 'src-tauri', 'resources', 'node-runtime', nodeDir),
  ];

  // 소스 파일 확인
  if (!fs.existsSync(sourcePath)) {
    throw new Error(`바이너리 파일을 찾을 수 없습니다: ${sourcePath}`);
  }

  // 각 타겟 디렉토리에 복사
  for (const targetDir of targetDirs) {
    fs.mkdirSync(targetDir, { recursive: true });
    const targetPath = path.join(targetDir, binaryName);
    fs.copyFileSync(sourcePath, targetPath);

    // Unix 시스템에서 실행 권한 설정
    if (process.platform !== 'win32') {
      fs.chmodSync(targetPath, 0o755);
    }

    console.log(`복사 완료: ${targetPath}`);
  }
};

// 메인 함수
const main = async () => {
  try {
    const platformInfo = getPlatformInfo();
    const { os, arch, extension } = platformInfo;

    const fileName = `node-${NODE_VERSION}-${os}-${arch}.${extension}`;
    const downloadUrl = `${BASE_URL}${fileName}`;

    console.log(`Node.js ${NODE_VERSION} 바이너리 다운로드 시작`);
    console.log(`플랫폼: ${os}-${arch}`);
    console.log(`파일: ${fileName}`);
    console.log(`URL: ${downloadUrl}\n`);

    // 임시 디렉토리 생성
    const tempDir = path.join(__dirname, '..', '.temp-node-download');
    fs.mkdirSync(tempDir, { recursive: true });
    const archivePath = path.join(tempDir, fileName);
    const extractDir = path.join(tempDir, 'extracted');

    // 다운로드
    await downloadFile(downloadUrl, archivePath);

    // 압축 해제
    fs.mkdirSync(extractDir, { recursive: true });
    await extractArchive(archivePath, extractDir);

    // 바이너리 복사
    await copyBinary(platformInfo, extractDir);

    // 임시 파일 정리
    console.log('\n임시 파일 정리 중...');
    fs.rmSync(tempDir, { recursive: true, force: true });

    console.log('\n✅ 모든 작업 완료!');
  } catch (error) {
    console.error('\n❌ 오류 발생:', error.message);
    process.exit(1);
  }
};

main();
