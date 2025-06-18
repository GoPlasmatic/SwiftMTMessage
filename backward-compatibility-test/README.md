# Backward Compatibility Test

This project tests backward compatibility between different versions of the `swift-mt-message` crate by parsing the same test data with both versions and comparing the JSON outputs.

## Overview

The backward compatibility test works by:

1. **Building with Published Version**: Uses the published version from crates.io (currently configured for v2.0.14)
2. **Generating Old JSON**: Parses all test data files and generates JSON output
3. **Building with Local Version**: Switches to the local development version
4. **Generating New JSON**: Parses the same test data with the new version
5. **Comparing Results**: Analyzes differences and reports compatibility status

## Quick Start

### Automated Test

Run the complete backward compatibility test with a single command:

```bash
./run_compatibility_test.sh
```

This script will:
- Build both versions
- Generate JSON outputs
- Compare the results
- Generate a detailed report

### Manual Testing

You can also run individual steps manually:

```bash
# Generate JSON with published version
cargo run --release --features published --bin generate_old_json

# Generate JSON with local version  
cargo run --release --features local --no-default-features --bin generate_new_json

# Compare the outputs
cargo run --release --bin compare_compatibility -- --detailed --output report.md
```

## Configuration

### Version Configuration

Update the version in `Cargo.toml` to test against different published versions:

```toml
[dependencies]
swift-mt-message = { version = "2.0.14", optional = true }  # Change this version
```

### Test Data

The test uses all `.txt` files from the `../test_data/` directory. Current test files include:

- `mt103_norm.txt` - Standard MT103 message
- `mt103_stp.txt` - MT103 STP variant
- `mt103_rejt.txt` - MT103 rejection message
- `mt103_retn.txt` - MT103 return message
- `mt202_core.txt` - Basic MT202 message
- `mt202_cov.txt` - MT202 cover message
- `mt205_core.txt` - Basic MT205 message
- And more...

## Output Structure

```
backward-compatibility-test/
├── output/
│   ├── old_version/          # JSON from published version
│   │   ├── mt103_norm.json
│   │   ├── mt103_stp.json
│   │   └── ...
│   └── new_version/          # JSON from local version
│       ├── mt103_norm.json
│       ├── mt103_stp.json
│       └── ...
├── compatibility_report.md   # Detailed comparison report
└── ...
```

## Comparison Analysis

The comparison tool analyzes differences and categorizes them:

### ✅ Compatible Changes
- **Field Added**: New optional fields (backward compatible)
- **Value Enhancement**: Enhanced field descriptions or metadata

### ⚠️ Warning Changes  
- **Value Changed**: Field values that changed but don't break structure
- **Array Length**: Array size differences that might affect behavior

### 🔴 Breaking Changes
- **Field Removed**: Fields that existed in old version but missing in new
- **Type Changed**: Field type changes (string to number, etc.)
- **Structure Changed**: Changes that would break existing code

## Compatibility Levels

- **Identical**: 100% identical JSON output
- **Compatible**: New fields added, no breaking changes  
- **Breaking**: Structural changes that could break existing code
- **Parse Error**: One version failed to parse the test data

## Interpreting Results

### Success Criteria
```
✅ No breaking changes detected!
Compatibility: 100.0%
```

### Failure Indicators
```
❌ BREAKING CHANGES DETECTED!
🔴 mt103_norm - Breaking changes detected
  🔴 FieldRemoved at field_52a
    Old: {"bic": "BANKBEBBXXX"}
    New: null
```

## Continuous Integration

Add this to your CI pipeline to catch compatibility regressions:

```yaml
- name: Backward Compatibility Test
  run: |
    cd backward-compatibility-test
    ./run_compatibility_test.sh
```

The script exits with code 1 if breaking changes are detected, failing the CI build.

## Troubleshooting

### Common Issues

**"Failed to build with published version"**
- The specified version might not exist on crates.io
- Update the version in `Cargo.toml` to a valid published version

**"Failed to parse SWIFT message"**  
- Test data might be invalid or corrupted
- New version might have stricter parsing rules
- Check the error messages for specific parsing failures

**"No files found in test_data"**
- Ensure the `../test_data/` directory exists relative to this project
- Check that test files have `.txt` extension

### Debug Mode

For detailed debugging, run individual commands with verbose output:

```bash
RUST_LOG=debug cargo run --bin generate_old_json
```

## Advanced Usage

### Custom Comparison Options

```bash
# Only show breaking changes
cargo run --bin compare_compatibility -- --breaking-only

# Generate detailed report
cargo run --bin compare_compatibility -- --detailed --output detailed_report.md

# Compare specific directories
cargo run --bin compare_compatibility -- --old-dir custom/old --new-dir custom/new
```

### Testing Specific Files

Modify the generator scripts to process only specific files by filtering in the `process_file` function.

## Contributing

When adding new test cases:

1. Add `.txt` files to `../test_data/`
2. Run the compatibility test to establish baseline
3. Ensure new functionality doesn't break existing parsing

## Maintenance

### Updating Reference Version

When releasing a new version:

1. Update the reference version in `Cargo.toml`
2. Run the compatibility test
3. Document any intentional breaking changes
4. Update this README if the test process changes

### Cleaning Up

Remove generated files:

```bash
rm -rf output/
rm compatibility_report.md
cargo clean
``` 