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
| `↑` / `↓` | Navigate list items |
| `PgUp` / `PgDn` | Navigate 10 items at once |
| `Enter` | View detailed information for selected item |
| `a-z`, `0-9` | Filter items (searches ID, parent, name, tiers, extension) |
| `Backspace` | Remove last filter character |
| `Esc` / `q` | Quit (closes popup if open, otherwise exits) |

### Interface

- **Header**: Shows active taxonomy (Product, Content, Audience)
- **Filter**: Type to search across all fields
- **Results Table**: Displays filtered items with ID, parent, name, tier hierarchy, and extension
- **Help Bar**: Shows available keyboard shortcuts

## Data Sources

Taxonomy data is embedded from:
- `product-2.0.tsv`
- `content-3.1.tsv`
- `audience-1.1.tsv`

Data is compiled into the binary; no external files required at runtime.

## Requirements

- Rust 2024 edition or later
- Terminal with ANSI color support
