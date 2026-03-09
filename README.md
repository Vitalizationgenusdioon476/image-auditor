# 🖼 Image Auditor

A blazing-fast Rust TUI tool that scans your HTML/PHTML/JS/TS codebase for image delivery issues — the kind flagged by Lighthouse and Core Web Vitals.

## Detected Issues

| Issue | Severity |
|---|---|
| Wrong format (PNG/JPG instead of WebP/AVIF) | ⚠ Warning |
| Missing `width` / `height` attributes (causes CLS) | ✖ Error |
| Missing `loading="lazy"` | ⚠ Warning |
| Oversized image file (>200 KiB, local images) | ✖ Error |
| Missing `srcset` / responsive images | ℹ Info |
| JSX `<Image>` missing `alt` attribute | ⚠ Warning |

## Scanned File Types

`html`, `phtml`, `htm`, `jsx`, `tsx`, `js`, `ts`, `vue`, `svelte`, `hbs`, `ejs`, `njk`, `php`

Automatically skips: `node_modules`, `.git`, `dist`, `build`, `.next`

## Install

```bash
cargo install --path .
```

## Usage

```bash
# Launch interactive TUI (menu to pick path)
image-auditor

# Scan a specific directory directly
image-auditor ./my-project
image-auditor /var/www/html
```

## TUI Controls

| Key | Action |
|---|---|
| `Enter` | Edit path / confirm / view detail |
| `↑ ↓` or `j k` | Navigate |
| `Tab / Shift+Tab` | Filter by issue category |
| `1` | Show all severities |
| `2` | Errors only |
| `3` | Warnings only |
| `4` | Info only |
| `s` | Save report to `image-audit-report.json` |
| `q / Esc` | Back / quit |

## Build

```bash
cargo build --release
./target/release/image-auditor
```
