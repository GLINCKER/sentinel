#!/usr/bin/env bash
#
# Release Script for Sentinel
# Built by Glincker (A GLINR Product)
#
# Creates platform-specific distribution packages
#
# Usage:
#   ./scripts/release.sh [VERSION]
#   ./scripts/release.sh 0.1.0

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
VERSION="${1:-}"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$PROJECT_ROOT/dist"
RELEASE_DIR="$PROJECT_ROOT/release"
OS_TYPE="$(uname -s)"

echo -e "${BLUE}🛡️  Sentinel Release Script${NC}"
echo -e "${BLUE}===========================${NC}\n"

# Validate version
if [[ -z "$VERSION" ]]; then
    echo -e "${RED}❌ Version required${NC}"
    echo "Usage: $0 VERSION"
    echo "Example: $0 0.1.0"
    exit 1
fi

# Validate version format (semantic versioning)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9]+)?$ ]]; then
    echo -e "${RED}❌ Invalid version format: $VERSION${NC}"
    echo "Expected format: MAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH-PRERELEASE"
    echo "Examples: 0.1.0, 1.2.3, 0.1.0-alpha"
    exit 1
fi

echo -e "${GREEN}Version:${NC} $VERSION"
echo -e "${GREEN}Platform:${NC} $OS_TYPE\n"

# Create release directory
rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# Build release binary
echo -e "${YELLOW}Building release binary...${NC}"
bash "$PROJECT_ROOT/scripts/build.sh" release

# Platform-specific packaging
case "$OS_TYPE" in
    Darwin)
        echo -e "\n${YELLOW}Creating macOS distribution...${NC}"

        # Create app bundle structure
        APP_NAME="Sentinel.app"
        APP_DIR="$RELEASE_DIR/$APP_NAME"
        mkdir -p "$APP_DIR/Contents/MacOS"
        mkdir -p "$APP_DIR/Contents/Resources"

        # Copy binary
        cp "$DIST_DIR/sentinel" "$APP_DIR/Contents/MacOS/sentinel"
        chmod +x "$APP_DIR/Contents/MacOS/sentinel"

        # Create Info.plist
        cat > "$APP_DIR/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Sentinel</string>
    <key>CFBundleDisplayName</key>
    <string>Sentinel</string>
    <key>CFBundleIdentifier</key>
    <string>com.glincker.sentinel</string>
    <key>CFBundleVersion</key>
    <string>$VERSION</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleExecutable</key>
    <string>sentinel</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
</dict>
</plist>
EOF

        # Create DMG
        echo -e "${YELLOW}Creating DMG...${NC}"
        DMG_NAME="Sentinel-${VERSION}-macOS.dmg"
        hdiutil create -volname "Sentinel" -srcfolder "$APP_DIR" -ov -format UDZO "$RELEASE_DIR/$DMG_NAME"

        echo -e "${GREEN}✓${NC} Created: $DMG_NAME"

        # Code signing (if certificates available)
        if command -v codesign &> /dev/null && [[ -n "${APPLE_SIGNING_IDENTITY:-}" ]]; then
            echo -e "${YELLOW}Signing application...${NC}"
            codesign --force --deep --sign "$APPLE_SIGNING_IDENTITY" "$APP_DIR"
            echo -e "${GREEN}✓${NC} Signed with: $APPLE_SIGNING_IDENTITY"

            # Notarization (requires Apple Developer account)
            if [[ -n "${APPLE_ID:-}" ]] && [[ -n "${APPLE_PASSWORD:-}" ]]; then
                echo -e "${YELLOW}Notarizing...${NC}"
                xcrun notarytool submit "$RELEASE_DIR/$DMG_NAME" \
                    --apple-id "$APPLE_ID" \
                    --password "$APPLE_PASSWORD" \
                    --team-id "${APPLE_TEAM_ID:-}" \
                    --wait
                echo -e "${GREEN}✓${NC} Notarized"
            fi
        else
            echo -e "${YELLOW}⚠️  Skipping code signing (APPLE_SIGNING_IDENTITY not set)${NC}"
        fi
        ;;

    Linux)
        echo -e "\n${YELLOW}Creating Linux distribution...${NC}"

        # Create AppImage
        APPDIR="$RELEASE_DIR/Sentinel.AppDir"
        mkdir -p "$APPDIR/usr/bin"
        mkdir -p "$APPDIR/usr/share/applications"
        mkdir -p "$APPDIR/usr/share/icons/hicolor/256x256/apps"

        # Copy binary
        cp "$DIST_DIR/sentinel" "$APPDIR/usr/bin/sentinel"
        chmod +x "$APPDIR/usr/bin/sentinel"

        # Create desktop file
        cat > "$APPDIR/usr/share/applications/sentinel.desktop" <<EOF
[Desktop Entry]
Name=Sentinel
Exec=sentinel
Icon=sentinel
Type=Application
Categories=Development;Utility;
Comment=Process Manager & System Monitor
Terminal=false
EOF

        # Create AppRun script
        cat > "$APPDIR/AppRun" <<'EOF'
#!/bin/bash
APPDIR="$(dirname "$(readlink -f "$0")")"
exec "$APPDIR/usr/bin/sentinel" "$@"
EOF
        chmod +x "$APPDIR/AppRun"

        # Download appimagetool if needed
        if ! command -v appimagetool &> /dev/null; then
            echo -e "${YELLOW}Downloading appimagetool...${NC}"
            wget -q https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage -O /tmp/appimagetool
            chmod +x /tmp/appimagetool
            APPIMAGETOOL="/tmp/appimagetool"
        else
            APPIMAGETOOL="appimagetool"
        fi

        # Build AppImage
        APPIMAGE_NAME="Sentinel-${VERSION}-x86_64.AppImage"
        ARCH=x86_64 "$APPIMAGETOOL" "$APPDIR" "$RELEASE_DIR/$APPIMAGE_NAME"

        echo -e "${GREEN}✓${NC} Created: $APPIMAGE_NAME"

        # Create .deb package
        echo -e "${YELLOW}Creating .deb package...${NC}"
        DEB_DIR="$RELEASE_DIR/sentinel-deb"
        mkdir -p "$DEB_DIR/DEBIAN"
        mkdir -p "$DEB_DIR/usr/local/bin"

        # Copy binary
        cp "$DIST_DIR/sentinel" "$DEB_DIR/usr/local/bin/sentinel"

        # Create control file
        cat > "$DEB_DIR/DEBIAN/control" <<EOF
Package: sentinel
Version: $VERSION
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Glincker <sentinel@glincker.com>
Description: Process Manager & System Monitor
 Sentinel is an open-source desktop application for managing
 development processes with real-time system monitoring.
 Built by Glincker (A GLINR Product).
Homepage: https://glincker.com/sentinel
EOF

        # Build .deb
        dpkg-deb --build "$DEB_DIR" "$RELEASE_DIR/sentinel_${VERSION}_amd64.deb"

        echo -e "${GREEN}✓${NC} Created: sentinel_${VERSION}_amd64.deb"
        ;;

    MINGW*|MSYS*)
        echo -e "\n${YELLOW}Creating Windows distribution...${NC}"

        # Copy binary
        cp "$DIST_DIR/sentinel.exe" "$RELEASE_DIR/sentinel.exe"

        # Create installer with NSIS (if available)
        if command -v makensis &> /dev/null; then
            echo -e "${YELLOW}Creating installer...${NC}"
            # TODO: Create NSIS installer script
            echo -e "${YELLOW}⚠️  NSIS installer script not implemented yet${NC}"
        fi

        # Create ZIP archive
        ZIP_NAME="Sentinel-${VERSION}-Windows.zip"
        cd "$RELEASE_DIR"
        zip -q "$ZIP_NAME" sentinel.exe

        echo -e "${GREEN}✓${NC} Created: $ZIP_NAME"

        # Code signing (if certificate available)
        if [[ -n "${WINDOWS_CERT:-}" ]]; then
            echo -e "${YELLOW}Signing executable...${NC}"
            signtool sign /f "$WINDOWS_CERT" /p "${WINDOWS_CERT_PASSWORD:-}" /tr http://timestamp.digicert.com /td sha256 /fd sha256 sentinel.exe
            echo -e "${GREEN}✓${NC} Signed"
        else
            echo -e "${YELLOW}⚠️  Skipping code signing (WINDOWS_CERT not set)${NC}"
        fi
        ;;

    *)
        echo -e "${RED}❌ Unsupported platform: $OS_TYPE${NC}"
        exit 1
        ;;
esac

# Generate checksums
echo -e "\n${YELLOW}Generating checksums...${NC}"
cd "$RELEASE_DIR"
for file in *; do
    if [[ -f "$file" ]] && [[ "$file" != "*.sha256" ]]; then
        sha256sum "$file" > "$file.sha256"
    fi
done
echo -e "${GREEN}✓${NC} Checksums generated"

# Summary
echo -e "\n${GREEN}✅ Release build complete!${NC}"
echo -e "${BLUE}Version:${NC} $VERSION"
echo -e "${BLUE}Platform:${NC} $OS_TYPE"
echo -e "${BLUE}Output:${NC} $RELEASE_DIR/\n"

ls -lh "$RELEASE_DIR"

echo -e "\n${YELLOW}Next steps:${NC}"
echo "1. Test the release package"
echo "2. Create GitHub release: gh release create v${VERSION}"
echo "3. Upload artifacts: gh release upload v${VERSION} release/*"
echo "4. Update Homebrew formula (macOS)"
echo "5. Announce on Discord/Twitter"
