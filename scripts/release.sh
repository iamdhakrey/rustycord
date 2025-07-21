#!/bin/bash

# RustyCord Release Script
# Usage: ./scripts/release.sh [patch|minor|major|prerelease] [--dry-run] [--skip-github] [--skip-cargo]

set -e

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CARGO_TOML_PATH="$PROJECT_ROOT/Cargo.toml"
CHANGELOG_PATH="$PROJECT_ROOT/CHANGELOG.md"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Utility functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

log_step() {
    echo -e "${PURPLE}🔧 $1${NC}"
}

# Show usage
show_usage() {
    cat << EOF
🦀 RustyCord Release Script

Usage: ./scripts/release.sh [type] [options]

Release Types:
  patch     - 0.1.0 → 0.1.1 (bug fixes)
  minor     - 0.1.0 → 0.2.0 (new features)
  major     - 0.1.0 → 1.0.0 (breaking changes)
  prerelease - 0.1.0 → 0.1.1-alpha.1

Options:
  --dry-run        Show what would be done without making changes
  --skip-github    Skip GitHub release creation
  --skip-cargo     Skip Cargo publishing
  --help           Show this help message

Examples:
  ./scripts/release.sh patch
  ./scripts/release.sh minor --dry-run
  ./scripts/release.sh prerelease --skip-github
  ./scripts/release.sh major --skip-cargo
EOF
}

# Get current version from Cargo.toml
get_current_version() {
    grep '^version = ' "$CARGO_TOML_PATH" | sed 's/version = "\(.*\)"/\1/'
}

# Calculate next version
get_next_version() {
    local current_version="$1"
    local release_type="$2"
    
    # Split version into parts
    local major minor patch prerelease
    
    if [[ $current_version =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)(-alpha\.([0-9]+))?$ ]]; then
        major="${BASH_REMATCH[1]}"
        minor="${BASH_REMATCH[2]}"
        patch="${BASH_REMATCH[3]}"
        prerelease="${BASH_REMATCH[5]}"
    else
        log_error "Invalid version format: $current_version"
        exit 1
    fi
    
    case "$release_type" in
        "major")
            echo "$((major + 1)).0.0"
            ;;
        "minor")
            echo "$major.$((minor + 1)).0"
            ;;
        "patch")
            echo "$major.$minor.$((patch + 1))"
            ;;
        "prerelease")
            if [[ -n "$prerelease" ]]; then
                echo "$major.$minor.$patch-alpha.$((prerelease + 1))"
            else
                echo "$major.$minor.$((patch + 1))-alpha.1"
            fi
            ;;
        *)
            log_error "Unknown release type: $release_type"
            exit 1
            ;;
    esac
}

# Update Cargo.toml version
update_cargo_version() {
    local new_version="$1"
    local dry_run="$2"
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "Would update Cargo.toml version to $new_version"
        return
    fi
    
    # Create backup
    cp "$CARGO_TOML_PATH" "$CARGO_TOML_PATH.bak"
    
    # Update version in Cargo.toml
    sed -i "s/^version = \".*\"/version = \"$new_version\"/" "$CARGO_TOML_PATH"
    
    log_success "Updated Cargo.toml version to $new_version"
}

# Get git commits since last tag
get_commits_since_last_tag() {
    if git describe --tags --abbrev=0 >/dev/null 2>&1; then
        local last_tag
        last_tag=$(git describe --tags --abbrev=0)
        git log "$last_tag..HEAD" --oneline | sed 's/^[a-f0-9]\{7\} //'
    else
        # If no tags exist, get all commits
        git log --oneline | sed 's/^[a-f0-9]\{7\} //'
    fi
}

# Generate changelog entry
generate_changelog_entry() {
    local version="$1"
    local commits="$2"
    local date
    date=$(date +%Y-%m-%d)
    
    local features=()
    local fixes=()
    local changes=()
    local breaking=()
    
    while IFS= read -r commit; do
        if [[ "$commit" =~ ^feat(\(.+\))?:.*$ ]] || [[ "$commit" =~ ^✨.*$ ]]; then
            features+=("$commit")
        elif [[ "$commit" =~ ^fix(\(.+\))?:.*$ ]] || [[ "$commit" =~ ^🐛.*$ ]]; then
            fixes+=("$commit")
        elif [[ "$commit" =~ ^BREAKING.*$ ]] || [[ "$commit" =~ .*BREAKING\ CHANGE.*$ ]]; then
            breaking+=("$commit")
        else
            changes+=("$commit")
        fi
    done <<< "$commits"
    
    local entry="## [$version] - $date\n\n"
    
    if [[ ${#breaking[@]} -gt 0 ]]; then
        entry+="### 💥 BREAKING CHANGES\n"
        for item in "${breaking[@]}"; do
            entry+="- $item\n"
        done
        entry+="\n"
    fi
    
    if [[ ${#features[@]} -gt 0 ]]; then
        entry+="### ✨ Features\n"
        for item in "${features[@]}"; do
            entry+="- $item\n"
        done
        entry+="\n"
    fi
    
    if [[ ${#fixes[@]} -gt 0 ]]; then
        entry+="### 🐛 Bug Fixes\n"
        for item in "${fixes[@]}"; do
            entry+="- $item\n"
        done
        entry+="\n"
    fi
    
    if [[ ${#changes[@]} -gt 0 ]]; then
        entry+="### 📝 Other Changes\n"
        for item in "${changes[@]}"; do
            entry+="- $item\n"
        done
        entry+="\n"
    fi
    
    echo -e "$entry"
}

# Update CHANGELOG.md
update_changelog() {
    local version="$1"
    local commits="$2"
    local dry_run="$3"
    
    local new_entry
    new_entry=$(generate_changelog_entry "$version" "$commits")
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "Would update CHANGELOG.md with:"
        echo -e "$new_entry"
        return
    fi
    
    if [[ ! -f "$CHANGELOG_PATH" ]]; then
        cat > "$CHANGELOG_PATH" << EOF
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

EOF
    fi
    
    # Create backup
    cp "$CHANGELOG_PATH" "$CHANGELOG_PATH.bak"
    
    # Insert new entry after the header
    local temp_file
    temp_file=$(mktemp)
    
    # Read the header
    sed '/^## \[/,$d' "$CHANGELOG_PATH" > "$temp_file"
    
    # Add the new entry
    echo -e "$new_entry" >> "$temp_file"
    
    # Add existing entries
    sed -n '/^## \[/,$p' "$CHANGELOG_PATH" >> "$temp_file"
    
    # Replace the original file
    mv "$temp_file" "$CHANGELOG_PATH"
    
    log_success "Updated CHANGELOG.md with version $version"
}

# Run tests
run_tests() {
    local dry_run="$1"
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "Would run: cargo test"
        return
    fi
    
    log_step "Running tests..."
    if cargo test; then
        log_success "All tests passed"
    else
        log_error "Tests failed. Aborting release."
        exit 1
    fi
}

# Build and check
build_and_check() {
    local dry_run="$1"
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "Would run: cargo check && cargo build --release"
        return
    fi
    
    log_step "Running cargo check..."
    if cargo check; then
        log_success "Cargo check passed"
    else
        log_error "Cargo check failed. Aborting release."
        exit 1
    fi
    
    log_step "Building release version..."
    if cargo build --release; then
        log_success "Release build completed"
    else
        log_error "Release build failed. Aborting release."
        exit 1
    fi
}

# Create git tag
create_git_tag() {
    local version="$1"
    local dry_run="$2"
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "Would run: git add . && git commit -m 'chore: release v$version' && git tag -a v$version -m 'Release v$version'"
        return
    fi
    
    log_step "Creating git commit and tag..."
    git add .
    git commit -m "chore: release v$version"
    git tag -a "v$version" -m "Release v$version"
    
    log_success "Created git tag v$version"
}

# Push to remote
push_to_remote() {
    local dry_run="$1"
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "Would run: git push origin main && git push origin --tags"
        return
    fi
    
    log_step "Pushing to remote repository..."
    git push origin main
    git push origin --tags
    
    log_success "Pushed to remote repository"
}

# Publish to Cargo
publish_to_cargo() {
    local dry_run="$1"
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "Would run: cargo publish"
        return
    fi
    
    log_step "Publishing to crates.io..."
    if cargo publish --allow-dirty; then
        log_success "Published to crates.io"
    else
        log_error "Failed to publish to crates.io"
        exit 1
    fi
}

# Create GitHub release
create_github_release() {
    local version="$1"
    local commits="$2"
    local dry_run="$3"
    
    local release_notes
    release_notes=$(generate_changelog_entry "$version" "$commits" | sed 's/^## \[.*\] - .*$//')
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "Would create GitHub release v$version with notes:"
        echo -e "$release_notes"
        return
    fi
    
    log_step "Creating GitHub release..."
    
    # Check if gh CLI is available
    if ! command -v gh >/dev/null 2>&1; then
        log_warning "GitHub CLI (gh) not available. Skipping GitHub release."
        log_info "Manual release creation required at: https://github.com/iamdhakrey/rustcord/releases/new"
        return
    fi
    
    # Create release notes file
    local release_notes_file
    release_notes_file=$(mktemp)
    echo -e "$release_notes" > "$release_notes_file"
    
    # Determine if it's a prerelease
    local prerelease_flag=""
    if [[ "$version" =~ .*-alpha\..*$ ]] || [[ "$version" =~ .*-beta\..*$ ]] || [[ "$version" =~ .*-rc\..*$ ]]; then
        prerelease_flag="--prerelease"
    fi
    
    # Create the release
    if gh release create "v$version" --title "Release v$version" --notes-file "$release_notes_file" $prerelease_flag; then
        log_success "Created GitHub release v$version"
    else
        log_error "Failed to create GitHub release"
        log_info "Manual release creation required at: https://github.com/iamdhakrey/rustcord/releases/new"
    fi
    
    # Clean up
    rm "$release_notes_file"
}

# Main release function
main_release() {
    local release_type="$1"
    local dry_run="$2"
    local skip_github="$3"
    local skip_cargo="$4"
    
    log_info "🚀 Starting $release_type release for RustyCord..."
    
    # Get current version and calculate next version
    local current_version
    current_version=$(get_current_version)
    local next_version
    next_version=$(get_next_version "$current_version" "$release_type")
    
    log_info "📈 Version: $current_version → $next_version"
    
    if [[ "$dry_run" == "true" ]]; then
        log_warning "🧪 DRY RUN - No changes will be made"
    fi
    
    # Get commits for changelog
    local commits
    commits=$(get_commits_since_last_tag)
    local commit_count
    commit_count=$(echo "$commits" | wc -l)
    
    if [[ -n "$commits" ]]; then
        log_info "📋 Found $commit_count commits since last release"
    else
        log_info "📋 No commits found since last release"
    fi
    
    # Pre-release checks
    if [[ "$dry_run" != "true" ]]; then
        log_step "Running pre-release checks..."
        
        # Check if working directory is clean
        if [[ -n $(git status --porcelain) ]]; then
            log_error "Working directory is not clean. Please commit or stash changes."
            exit 1
        fi
        
        # Check if we're on main branch
        local current_branch
        current_branch=$(git branch --show-current)
        if [[ "$current_branch" != "main" ]]; then
            log_warning "Not on main branch (current: $current_branch). Continue? (y/N)"
            read -r response
            if [[ ! "$response" =~ ^[Yy]$ ]]; then
                log_info "Aborting release."
                exit 0
            fi
        fi
    fi
    
    # Run tests and build
    run_tests "$dry_run"
    build_and_check "$dry_run"
    
    # Update version and changelog
    update_cargo_version "$next_version" "$dry_run"
    update_changelog "$next_version" "$commits" "$dry_run"
    
    # Git operations
    create_git_tag "$next_version" "$dry_run"
    
    if [[ "$skip_github" != "true" ]]; then
        push_to_remote "$dry_run"
        create_github_release "$next_version" "$commits" "$dry_run"
    fi
    
    # Publish to Cargo
    if [[ "$skip_cargo" != "true" ]]; then
        if [[ "$next_version" =~ .*-alpha\..*$ ]]; then
            log_warning "Prerelease version detected. Skipping Cargo publish."
            log_info "To publish prerelease to crates.io, run: cargo publish"
        else
            publish_to_cargo "$dry_run"
        fi
    fi
    
    log_success "🎉 Release v$next_version completed successfully!"
    
    if [[ "$dry_run" == "true" ]]; then
        log_info "This was a dry run. No changes were made."
    else
        log_info "🔗 Release URL: https://github.com/iamdhakrey/rustcord/releases/tag/v$next_version"
        if [[ "$skip_cargo" != "true" && ! "$next_version" =~ .*-alpha\..*$ ]]; then
            log_info "📦 Crates.io URL: https://crates.io/crates/rustycord"
        fi
    fi
}

# Parse command line arguments
parse_args() {
    local release_type=""
    local dry_run="false"
    local skip_github="false"
    local skip_cargo="false"
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            patch|minor|major|prerelease)
                release_type="$1"
                shift
                ;;
            --dry-run)
                dry_run="true"
                shift
                ;;
            --skip-github)
                skip_github="true"
                shift
                ;;
            --skip-cargo)
                skip_cargo="true"
                shift
                ;;
            --help|-h)
                show_usage
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    if [[ -z "$release_type" ]]; then
        log_error "Release type is required"
        show_usage
        exit 1
    fi
    
    main_release "$release_type" "$dry_run" "$skip_github" "$skip_cargo"
}

# Check dependencies
check_dependencies() {
    local missing_deps=()
    
    if ! command -v git >/dev/null 2>&1; then
        missing_deps+=("git")
    fi
    
    if ! command -v cargo >/dev/null 2>&1; then
        missing_deps+=("cargo")
    fi
    
    if [[ ${#missing_deps[@]} -gt 0 ]]; then
        log_error "Missing required dependencies: ${missing_deps[*]}"
        exit 1
    fi
}

# Main entry point
main() {
    check_dependencies
    parse_args "$@"
}

# Run the script
main "$@"
