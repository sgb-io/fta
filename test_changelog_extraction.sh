#!/bin/bash

# Changelog Extraction Test Script
# 
# This script validates the changelog extraction functionality used in the GitHub release workflow.
# It tests the awk-based logic that extracts changelog content for specific versions from CHANGELOG.md.
#
# Test cases covered:
# - Existing versions (v3.0.0, v2.0.2, v1.0.0, etc.)
# - Non-existent versions (should return empty)
# - Partial matches (should not match, e.g., "v3.0" != "v3.0.0")
# - Fallback mechanism for missing versions
# - Current version from Cargo.toml
#
# The script replicates the exact awk logic from .github/workflows/release.yml
# including handling Windows line endings and precise version matching.

set -e  # Exit on any error

echo "🧪 Testing changelog extraction functionality..."

# Create a function that mirrors the logic from the release workflow
extract_changelog_content() {
    local version="$1"
    local changelog_file="$2"
    
    if [ ! -f "$changelog_file" ]; then
        echo "Error: Changelog file $changelog_file not found"
        return 1
    fi
    
    # This is the exact same awk script from the release workflow (handles Windows line endings)
    local content=$(awk -v version="v$version" '
        BEGIN { found=0; content="" }
        /^## / {
            if (found) exit
            # Remove any trailing carriage return and match exactly
            line = $0
            gsub(/\r$/, "", line)
            if (line == "## " version) {
                found=1
                next
            }
        }
        found && /^## / { exit }
        found { 
            if (content != "") content = content "\n"
            content = content $0 
        }
        END { print content }
    ' "$changelog_file")
    
    echo "$content"
}

# Function to run a test case
run_test() {
    local test_name="$1"
    local version="$2"
    local expected_pattern="$3"
    local should_contain="$4"  # "true" if content should be found, "false" if empty
    
    echo ""
    echo "📋 Test: $test_name"
    echo "   Version: v$version"
    
    local result=$(extract_changelog_content "$version" "CHANGELOG.md")
    
    if [ "$should_contain" = "true" ]; then
        if [ -n "$result" ]; then
            if echo "$result" | grep -q "$expected_pattern"; then
                echo "   ✅ PASS: Found expected content containing '$expected_pattern'"
                echo "   📄 Content:"
                echo "$result" | sed 's/^/      /'
            else
                echo "   ❌ FAIL: Content found but doesn't contain expected pattern '$expected_pattern'"
                echo "   📄 Actual content:"
                echo "$result" | sed 's/^/      /'
                return 1
            fi
        else
            echo "   ❌ FAIL: Expected content but got empty result"
            return 1
        fi
    else
        if [ -z "$result" ]; then
            echo "   ✅ PASS: Correctly returned empty content for non-existent version"
        else
            echo "   ❌ FAIL: Expected empty content but got:"
            echo "$result" | sed 's/^/      /'
            return 1
        fi
    fi
}

# Test cases
echo "Running test suite against CHANGELOG.md..."

# Test existing versions
run_test "Extract v3.0.0 content" "3.0.0" "Breaking changes" "true"
run_test "Extract v2.0.2 content" "2.0.2" "Enabled support for TypeScript decorators" "true"
run_test "Extract v2.0.1 content" "2.0.1" "Added type definition to NPM package" "true"
run_test "Extract v1.0.0 content" "1.0.0" "Breaking changes" "true"
run_test "Extract v0.2.0 content" "0.2.0" "Potentially breaking" "true"

# Test edge cases
run_test "Non-existent version" "99.99.99" "" "false"
run_test "Partial version match should not work" "3.0" "" "false"

# Test the fallback mechanism (simulating what the workflow does)
echo ""
echo "🔄 Testing fallback mechanism..."
test_version="99.99.99"
changelog_content=$(extract_changelog_content "$test_version" "CHANGELOG.md")

if [ -z "$changelog_content" ]; then
    fallback_content="Release v$test_version

See [CHANGELOG.md](./CHANGELOG.md) for details."
    echo "✅ PASS: Fallback mechanism works correctly"
    echo "📄 Fallback content:"
    echo "$fallback_content" | sed 's/^/   /'
else
    echo "❌ FAIL: Fallback mechanism test failed - should have been empty"
    exit 1
fi

# Test that the script handles the actual Cargo.toml version extraction
echo ""
echo "🔧 Testing version extraction from Cargo.toml..."
if [ -f "crates/fta/Cargo.toml" ]; then
    cargo_version=$(grep '^version =' crates/fta/Cargo.toml | sed 's/^version = "\(.*\)"/\1/')
    echo "📦 Current version in Cargo.toml: $cargo_version"
    
    # Try to extract changelog for current version
    current_content=$(extract_changelog_content "$cargo_version" "CHANGELOG.md")
    if [ -n "$current_content" ]; then
        echo "✅ PASS: Successfully found changelog content for current version v$cargo_version"
        echo "📄 Content preview:"
        echo "$current_content" | head -3 | sed 's/^/   /'
    else
        echo "⚠️  WARNING: No changelog content found for current version v$cargo_version"
        echo "   This might be expected if the current version hasn't been released yet"
    fi
else
    echo "❌ ERROR: crates/fta/Cargo.toml not found"
    exit 1
fi

echo ""
echo "🎉 All tests completed successfully!"
echo ""
echo "This validates that the changelog extraction logic in .github/workflows/release.yml"
echo "will work correctly when creating GitHub releases."