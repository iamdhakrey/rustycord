# Brand Assets

<div align="center">
  <img src="/images/rustycord.png" alt="rustycord Full Logo" width="256" height="256">
  
</div>

This page contains the official rustycord brand assets for use in documentation, presentations, and projects.

!!! info "Usage Guidelines"
    These assets are provided for use in rustycord-related projects and documentation. Please maintain the visual integrity of the brand when using these assets.

## Available Assets

### Full Logo (1024x1024)
- **File**: `docs/images/rustycord.png`
- **Dimensions**: 1024 × 1024 pixels
- **Use**: Main logo for large displays, presentations, and banners

<div align="center">
  <img src="/images/rustycord.png" alt="rustycord Full Logo" width="200" height="200">
</div>

### Medium Logo (128x128)
- **File**: `docs/images/rustycord-logo.png`
- **Dimensions**: 128 × 128 pixels
- **Use**: Documentation headers, medium-sized displays

<div align="center">
  <img src="/images/rustycord-logo.png" alt="rustycord Medium Logo" width="128" height="128">
</div>

### Icon (64x64)
- **File**: `docs/images/rustycord-icon.png`
- **Dimensions**: 64 × 64 pixels
- **Use**: Navigation bars, small displays, application icons

<div align="center">
  <img src="/images/rustycord-icon.png" alt="rustycord Icon" width="64" height="64">
</div>

### Favicon (32x32)
- **File**: `docs/images/rustycord-favicon.png`
- **Dimensions**: 32 × 32 pixels
- **Use**: Browser favicons, very small displays, status indicators

<div align="center">
  <img src="/images/rustycord-favicon.png" alt="rustycord Favicon" width="32" height="32">
</div>

## Usage Examples

### Markdown
```markdown
# With centered logo
<div align="center">
  <img src="docs/images/rustycord-logo.png" alt="rustycord Logo" width="128" height="128">
  
  # Your Project Title
</div>

# Inline with text
![rustycord](/images/rustycord-favicon.png) rustycord Project
```

### HTML
```html
<!-- Centered logo -->
<div align="center">
  <img src="/images/rustycord-logo.png" alt="rustycord Logo" width="128" height="128">
  <h1>Your Project Title</h1>
</div>

<!-- Inline logo -->
<img src="/images/rustycord-favicon.png" alt="rustycord" width="32" height="32"> rustycord Project
```

### MkDocs Configuration
```yaml
theme:
  name: material
  logo: images/rustycord-logo.png
  favicon: images/rustycord-favicon.png
```

!!! tip "Responsive Design"
    Use different sized assets based on the display context:
    
    - **Desktop headers**: Use 128px logo
    - **Mobile/responsive**: Use 64px icon
    - **Browser tabs**: Use 32px favicon
    - **Large banners**: Use full 1024px version

## File Formats

All assets are provided in PNG format with transparent backgrounds for maximum compatibility and flexibility.

---

<div align="center">
  <img src="/images/rustycord-favicon.png" alt="rustycord" width="24" height="24">
  <em>rustycord - A Discord bot library for Rust</em>
</div>
