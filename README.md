# IAB Taxonomy Browser

Terminal user interface for browsing IAB (Interactive Advertising Bureau) taxonomy data: Product (v2.0), Content (v3.1), and Audience (v1.1).

## Build

```bash
cargo build --release
```

Binary output: `target/release/iab`

## Installation

```bash
cargo install --path .
```

Or copy the binary from `target/release/iab` to a location in your PATH.

## Usage

```bash
iab
```

The application launches a full-screen TUI with three taxonomy datasets.

### Controls

| Key | Action |
|-----|--------|
| `Tab` | Switch to next taxonomy (Product → Content → Audience) |
| `Shift+Tab` | Switch to previous taxonomy |
| `↑` / `↓` | Navigate tree items |
| `←` / `→` | Collapse / Expand selected node |
| `Ctrl+Space` | Toggle expand/collapse for selected node |
| `PgUp` / `PgDn` | Navigate 10 items at once |
| `Enter` | View detailed information for selected item |
| Type characters | Filter items (searches ID, name, tiers, extension) - supports spaces |
| `Backspace` | Remove last filter character |
| `Esc` / `q` | Quit (closes popup if open, otherwise exits) |

### Interface

- **Header**: Shows active taxonomy (Product, Content, Audience) with tab navigation
- **Filter**: Type to search across all fields (e.g., "home insurance")
  - Matching text is highlighted with yellow background
  - Filtered results automatically expand to show full hierarchy
- **Tree View**: Hierarchical display showing parent-child relationships
  - IDs displayed in bold
  - `▶` / `▼` symbols indicate collapsed/expanded nodes
  - Shows all ancestors and descendants of matching items when filtering
- **Scrollbar**: Indicates current position in the visible tree
- **Help Bar**: Shows available keyboard shortcuts

## Features

- **Hierarchical Tree Display**: Navigate parent-child relationships naturally with expand/collapse
- **Smart Filtering**: Type multi-word queries like "home insurance" to find items
  - Matches highlighted with yellow background for easy identification
  - Automatically expands tree to show matching items with full context
- **Fast Navigation**: Arrow keys, PageUp/PageDown, and Ctrl+Space for efficient browsing
- **Data Integrity**: Handles circular references in source data gracefully

## Data Sources

Taxonomy data is embedded from:
- `product-2.0.tsv` (IAB Product Taxonomy v2.0)
- `content-3.1.tsv` (IAB Content Taxonomy v3.1)
- `audience-1.1.tsv` (IAB Audience Taxonomy v1.1)

Data is compiled into the binary; no external files required at runtime.

## Requirements

- Rust 2024 edition or later
- Terminal with ANSI color support
