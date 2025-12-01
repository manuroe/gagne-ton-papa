# web-test

CodSpeed benchmark testing experiment for the gagne-ton-papa project.

## Overview

This project follows the [CodSpeed Vitest tutorial](https://codspeed.io/blog/vitest-bench-performance-regressions) to set up performance regression tracking using Vitest benchmarks.

## Setup

The project includes:
- **Vitest** for benchmarking
- **@codspeed/vitest-plugin** for consistent performance measurements
- **TypeScript** for type safety
- **GitHub Actions** workflow for CI integration

## Running Benchmarks

Install dependencies:
```bash
npm install
```

Run benchmarks locally:
```bash
npm run bench
```

## What's Included

### Source Code
- `src/fibonacci.ts` - Simple recursive Fibonacci implementation (from tutorial)
- `src/fibonacci.bench.ts` - Benchmark tests for the Fibonacci function

### Configuration
- `vitest.config.ts` - Vitest configuration with CodSpeed plugin
- `tsconfig.json` - TypeScript configuration
- `package.json` - Dependencies and scripts

### CI/CD
- `.github/workflows/codspeed.yml` - GitHub Actions workflow for running benchmarks

## How It Works

1. When you push to `main` or open a pull request, the GitHub Actions workflow runs
2. CodSpeed instruments the benchmark code and runs it in a consistent environment
3. Results are published to [CodSpeed.io](https://codspeed.io)
4. Performance changes are reported in pull requests with flame graphs and metrics

## CodSpeed Integration

The benchmarks are automatically run in CI and published to CodSpeed. To view the results:

1. Enable the repository on [CodSpeed.io](https://codspeed.io)
2. Install the CodSpeed GitHub App
3. Push changes or open a pull request
4. View performance reports in PR comments and on the CodSpeed dashboard

## Next Steps

This is an experimental setup to learn how to use CodSpeed for performance regression tracking. Future improvements could include:
- Benchmarking actual game solver logic
- Performance testing for WASM bindings
- Frontend rendering benchmarks
