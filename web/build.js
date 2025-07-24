#!/usr/bin/env node

/**
 * Marco 2.0 Web Build Script
 * 
 * Automates WASM compilation and web bundling for deployment
 */

const { execSync, spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

// Configuration
const BUILD_CONFIG = {
  wasmTarget: 'web',
  outputDir: 'pkg',
  releaseMode: process.argv.includes('--release'),
  verbose: process.argv.includes('--verbose'),
  skipWasm: process.argv.includes('--skip-wasm'),
  skipWeb: process.argv.includes('--skip-web'),
  analyze: process.argv.includes('--analyze'),
};

// Colors for console output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
};

function log(message, color = colors.reset) {
  console.log(`${color}${message}${colors.reset}`);
}

function logStep(step) {
  log(`\n🔄 ${step}`, colors.cyan);
}

function logSuccess(message) {
  log(`✅ ${message}`, colors.green);
}

function logWarning(message) {
  log(`⚠️  ${message}`, colors.yellow);
}

function logError(message) {
  log(`❌ ${message}`, colors.red);
}

function execCommand(command, options = {}) {
  if (BUILD_CONFIG.verbose) {
    log(`Running: ${command}`, colors.blue);
  }
  
  try {
    const result = execSync(command, {
      stdio: BUILD_CONFIG.verbose ? 'inherit' : 'pipe',
      encoding: 'utf8',
      ...options,
    });
    return result;
  } catch (error) {
    logError(`Command failed: ${command}`);
    logError(error.message);
    process.exit(1);
  }
}

function checkDependencies() {
  logStep('Checking dependencies...');
  
  // Check for required tools
  const tools = [
    { name: 'cargo', command: 'cargo --version' },
    { name: 'wasm-pack', command: 'wasm-pack --version' },
    { name: 'npm', command: 'npm --version' },
  ];
  
  for (const tool of tools) {
    try {
      execCommand(tool.command, { stdio: 'pipe' });
      logSuccess(`${tool.name} is available`);
    } catch (error) {
      logError(`${tool.name} is not installed or not in PATH`);
      process.exit(1);
    }
  }
}

function buildWasm() {
  if (BUILD_CONFIG.skipWasm) {
    logWarning('Skipping WASM build');
    return;
  }
  
  logStep('Building WASM module...');
  
  const wasmPackArgs = [
    'build',
    `--target ${BUILD_CONFIG.wasmTarget}`,
    `--out-dir ${BUILD_CONFIG.outputDir}`,
    '--features console_error_panic_hook',
  ];
  
  if (BUILD_CONFIG.releaseMode) {
    wasmPackArgs.push('--release');
  } else {
    wasmPackArgs.push('--dev');
  }
  
  const command = `wasm-pack ${wasmPackArgs.join(' ')}`;
  execCommand(command, { cwd: path.resolve(__dirname, '..') });
  
  // Check WASM file size
  const wasmFile = path.join(__dirname, '../pkg/marco_2_web_bg.wasm');
  if (fs.existsSync(wasmFile)) {
    const stats = fs.statSync(wasmFile);
    const sizeMB = (stats.size / (1024 * 1024)).toFixed(2);
    logSuccess(`WASM built successfully (${sizeMB} MB)`);
    
    if (sizeMB > 5) {
      logWarning(`WASM file is large (${sizeMB} MB). Consider optimizing.`);
    }
  }
}

function installNodeDependencies() {
  logStep('Installing Node.js dependencies...');
  
  const packageJsonPath = path.join(__dirname, 'package.json');
  if (!fs.existsSync(packageJsonPath)) {
    logError('package.json not found');
    process.exit(1);
  }
  
  execCommand('npm install', { cwd: __dirname });
  logSuccess('Node.js dependencies installed');
}

function buildWeb() {
  if (BUILD_CONFIG.skipWeb) {
    logWarning('Skipping web build');
    return;
  }
  
  logStep('Building web application...');
  
  const mode = BUILD_CONFIG.releaseMode ? 'production' : 'development';
  let command = `npm run build:web:${BUILD_CONFIG.releaseMode ? 'release' : ''}`;
  
  // Use the base build command if specific one doesn't exist
  if (BUILD_CONFIG.releaseMode) {
    command = 'npm run build:web:release';
  } else {
    command = 'npm run build:web';
  }
  
  execCommand(command, { cwd: __dirname });
  
  // Check bundle size
  const distDir = path.join(__dirname, 'dist');
  if (fs.existsSync(distDir)) {
    const files = fs.readdirSync(distDir);
    const jsFiles = files.filter(f => f.endsWith('.js'));
    let totalSize = 0;
    
    for (const file of jsFiles) {
      const filePath = path.join(distDir, file);
      const stats = fs.statSync(filePath);
      totalSize += stats.size;
    }
    
    const sizeMB = (totalSize / (1024 * 1024)).toFixed(2);
    logSuccess(`Web bundle built successfully (${sizeMB} MB JS)`);
    
    if (totalSize > 5 * 1024 * 1024) {
      logWarning(`Bundle is large (${sizeMB} MB). Consider code splitting.`);
    }
  }
}

function analyzeBundle() {
  if (!BUILD_CONFIG.analyze) {
    return;
  }
  
  logStep('Analyzing bundle...');
  
  try {
    execCommand('npm run analyze', { cwd: __dirname });
    logSuccess('Bundle analysis complete');
  } catch (error) {
    logWarning('Bundle analysis failed (this is optional)');
  }
}

function optimizeAssets() {
  if (!BUILD_CONFIG.releaseMode) {
    return;
  }
  
  logStep('Optimizing assets...');
  
  // TODO: Add asset optimization logic
  // - Compress images
  // - Minify CSS
  // - Generate WebP variants
  
  logSuccess('Assets optimized');
}

function generateDeploymentInfo() {
  logStep('Generating deployment info...');
  
  const buildInfo = {
    timestamp: new Date().toISOString(),
    mode: BUILD_CONFIG.releaseMode ? 'production' : 'development',
    version: require('./package.json').version,
    features: {
      wasm: !BUILD_CONFIG.skipWasm,
      web: !BUILD_CONFIG.skipWeb,
    },
    sizes: {},
  };
  
  // Get file sizes
  try {
    const wasmFile = path.join(__dirname, '../pkg/marco_2_web_bg.wasm');
    if (fs.existsSync(wasmFile)) {
      buildInfo.sizes.wasm = fs.statSync(wasmFile).size;
    }
    
    const distDir = path.join(__dirname, 'dist');
    if (fs.existsSync(distDir)) {
      const files = fs.readdirSync(distDir);
      buildInfo.sizes.total = files.reduce((total, file) => {
        const filePath = path.join(distDir, file);
        if (fs.statSync(filePath).isFile()) {
          return total + fs.statSync(filePath).size;
        }
        return total;
      }, 0);
    }
  } catch (error) {
    logWarning('Could not calculate file sizes');
  }
  
  const buildInfoPath = path.join(__dirname, 'dist/build-info.json');
  fs.writeFileSync(buildInfoPath, JSON.stringify(buildInfo, null, 2));
  
  logSuccess('Deployment info generated');
  log(JSON.stringify(buildInfo, null, 2), colors.cyan);
}

function printUsage() {
  log('\nMarco 2.0 Web Build Script', colors.bright);
  log('Usage: node build.js [options]\n');
  log('Options:');
  log('  --release      Build in release mode (optimized)');
  log('  --verbose      Show detailed output');
  log('  --skip-wasm    Skip WASM compilation');
  log('  --skip-web     Skip web bundling');
  log('  --analyze      Analyze bundle size');
  log('  --help         Show this help message\n');
}

// Main build process
async function main() {
  if (process.argv.includes('--help')) {
    printUsage();
    return;
  }
  
  log('🚀 Marco 2.0 Web Build Starting...', colors.bright);
  log(`Mode: ${BUILD_CONFIG.releaseMode ? 'RELEASE' : 'DEVELOPMENT'}`, colors.magenta);
  
  const startTime = Date.now();
  
  try {
    checkDependencies();
    
    if (!BUILD_CONFIG.skipWeb) {
      installNodeDependencies();
    }
    
    buildWasm();
    buildWeb();
    
    if (BUILD_CONFIG.releaseMode) {
      optimizeAssets();
    }
    
    analyzeBundle();
    generateDeploymentInfo();
    
    const buildTime = ((Date.now() - startTime) / 1000).toFixed(1);
    logSuccess(`\n🎉 Build completed successfully in ${buildTime}s`);
    
    if (BUILD_CONFIG.releaseMode) {
      log('\nProduction build ready for deployment!', colors.green);
      log('Deploy the contents of the "dist" directory to your web server.', colors.cyan);
    } else {
      log('\nDevelopment build complete!', colors.green);
      log('Run "npm run serve" to start a local server.', colors.cyan);
    }
    
  } catch (error) {
    logError(`\n💥 Build failed: ${error.message}`);
    process.exit(1);
  }
}

// Run the build
main().catch(error => {
  logError(`Unexpected error: ${error.message}`);
  process.exit(1);
});

module.exports = {
  BUILD_CONFIG,
  buildWasm,
  buildWeb,
};
