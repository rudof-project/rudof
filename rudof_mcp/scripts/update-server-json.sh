#!/bin/bash
# Script to update SHA-256 hashes in server.json from GitHub release artifacts
# Usage: ./update-server-json.sh <release-tag>
# Example: ./update-server-json.sh v0.2.7

set -e

# Check if release tag is provided
if [ -z "$1" ]; then
    echo "Error: Release tag not provided"
    echo "Usage: $0 <release-tag>"
    echo "Example: $0 v0.2.7"
    exit 1
fi

RELEASE_TAG="$1"
REPO_OWNER="rudof-project"
REPO_NAME="rudof"
SERVER_JSON="server.json"

# Check if server.json exists
if [ ! -f "$SERVER_JSON" ]; then
    echo "Error: server.json not found in current directory"
    exit 1
fi

echo "Updating server.json with SHA-256 hashes from release $RELEASE_TAG..."
echo ""

# GitHub release URL base
BASE_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/${RELEASE_TAG}/rudof-mcp"

# Platform configurations: platform_name, binary_suffix, placeholder
declare -a platforms=(
    "Linux x64:rudof-mcp_${RELEASE_TAG}_x86_64_linux_gnu:PLACEHOLDER_LINUX_X64_SHA256"
    "macOS x64:rudof-mcp_${RELEASE_TAG}_x86_64_apple:PLACEHOLDER_MACOS_X64_SHA256"
    "macOS ARM64:rudof-mcp_${RELEASE_TAG}_aarch64_apple:PLACEHOLDER_MACOS_ARM64_SHA256"
    "Windows x64:rudof-mcp_${RELEASE_TAG}_x86_64_windows_msvc.exe:PLACEHOLDER_WINDOWS_X64_SHA256"
)

# Create temporary directory
TMP_DIR=$(mktemp -d)
trap "rm -rf $TMP_DIR" EXIT

# Download and extract SHA-256 hashes
for platform_config in "${platforms[@]}"; do
    IFS=':' read -r platform_name binary_suffix placeholder <<< "$platform_config"
    
    echo "Processing $platform_name..."
    
    # Download SHA-256 file
    sha256_url="${BASE_URL}/${binary_suffix}.sha256"
    sha256_file="${TMP_DIR}/${binary_suffix}.sha256"
    
    if curl -L -f -s -o "$sha256_file" "$sha256_url"; then
        # Extract just the hash (first field)
        sha256_hash=$(awk '{print $1}' "$sha256_file")
        
        if [ -n "$sha256_hash" ]; then
            echo "  ✓ SHA-256: $sha256_hash"
            
            # Update server.json (cross-platform compatible)
            # Use sed or perl depending on what's available
            if command -v perl &> /dev/null; then
                perl -i -pe "s/$placeholder/$sha256_hash/g" "$SERVER_JSON"
            else
                # macOS requires sed -i '' while Linux requires sed -i
                if [[ "$OSTYPE" == "darwin"* ]]; then
                    sed -i '' "s/$placeholder/$sha256_hash/g" "$SERVER_JSON"
                else
                    sed -i "s/$placeholder/$sha256_hash/g" "$SERVER_JSON"
                fi
            fi
        else
            echo "  ✗ Failed to extract hash from $sha256_url"
        fi
    else
        echo "  ✗ Failed to download $sha256_url"
        echo "    (This is expected if the release doesn't exist yet)"
    fi
    echo ""
done

# Update version in server.json
# Remove 'v' prefix if present for version field
VERSION="${RELEASE_TAG#v}"
if command -v perl &> /dev/null; then
    perl -i -pe "s/\"version\": \"[^\"]+\"/\"version\": \"$VERSION\"/g" "$SERVER_JSON"
else
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/g" "$SERVER_JSON"
    else
        sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/g" "$SERVER_JSON"
    fi
fi

# Update all identifier URLs with the new release tag
if command -v perl &> /dev/null; then
    perl -i -pe "s|/releases/download/v[^/]+/|/releases/download/$RELEASE_TAG/|g" "$SERVER_JSON"
else
    if [[ "$OSTYPE" == "darwin"* ]]; then
        sed -i '' "s|/releases/download/v[^/]*/|/releases/download/$RELEASE_TAG/|g" "$SERVER_JSON"
    else
        sed -i "s|/releases/download/v[^/]*/|/releases/download/$RELEASE_TAG/|g" "$SERVER_JSON"
    fi
fi

echo "✓ server.json updated successfully!"
echo ""
echo "Summary:"
echo "  - Version: $VERSION"
echo "  - Release tag: $RELEASE_TAG"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git diff $SERVER_JSON"
echo "  2. Test the server.json: mcp-publisher init (if you have mcp-publisher installed)"
echo "  3. Commit the changes: git add $SERVER_JSON && git commit -m 'chore: update server.json hashes for $RELEASE_TAG'"
