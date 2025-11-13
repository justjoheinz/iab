# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

IAB Taxonomy Browser - A terminal user interface (TUI) application for browsing Interactive Advertising Bureau (IAB) taxonomy data across three datasets: Product (v2.0), Content (v3.1), and Audience (v1.1). Built with Rust using ratatui and tui-tree-widget for hierarchical tree display.

## Build Commands

```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Run the application
cargo run --release
```

## Architecture

### Single-File Structure

The entire application is contained in `src/main.rs` (~980 lines). This monolithic structure is intentional for simplicity.

### Core Components

**Data Model** (lines ~17-218):
- `TaxonomyItem` trait: Unified interface for all three taxonomy types
- Three structs: `Content`, `Product`, `Audience` - each with different tier depths (4, 3, 6 respectively)
- All implement `TaxonomyItem` and require `Clone` for tree construction
- Data embedded at compile-time via `include_str!` from TSV files

**Application State** (lines ~327-338):
- `App` struct holds:
  - Current datasource (Product/Content/Audience enum)
  - Filter input string
  - All three taxonomy datasets in memory
  - `TreeState<String>` from tui-tree-widget (manages selection, open/closed nodes)
  - Popup state for detail view

**Tree Building** (lines ~641-690):
- `build_tree_items()`: Converts flat parent-child data into hierarchical `TreeItem` structures
- Handles self-references as root nodes (IDs 1000, 1037 in product.tsv)
- `build_tree_recursive()`: Recursively constructs tree with proper display formatting
- Each tree node shows: **Bold ID** + name, with filter matches highlighted in yellow

**Filtering Logic** (lines ~365-438):
- `filtered_tree_items()`: Main entry point - builds full tree or filtered tree
- `filtered_tree_from_items()`: Shows matches + all ancestors + all descendants (comprehensive path visibility)
- Circular reference protection in both ancestor and descendant traversal
- Auto-expands all filtered nodes when filter is active

**Navigation** (lines ~564-638):
- TreeState-based navigation (Up/Down/Left/Right/PageUp/PageDown)
- **Ctrl+Space**: Toggle node expand/collapse (Space reserved for filter input)
- Filter input accepts all characters including spaces ("home insurance")
- Tab/Shift+Tab switches between datasources

**Scrollbar** (lines ~737-811, 853-892):
- `calculate_flat_index()`: Converts tree selection to flat list position
- `count_visible_tree_items()`: Counts only currently visible (expanded) nodes
- Scrollbar accurately tracks position as user navigates collapsed/expanded tree

**Rendering** (lines ~813-901):
- ratatui-based TUI with 4 sections: header tabs, filter input, tree view, help bar
- `highlight_match()`: Yellow background highlighting for filter matches in ID and name
- Detail popup shows full item information

## Key Implementation Details

### Circular Reference Handling

The data files contain self-referencing items (ID=ParentID). Multiple protections:
1. Tree building: Self-references treated as root nodes (line ~648)
2. Ancestor traversal: Visited set prevents infinite loops (line ~412)
3. Descendant traversal: Checks if ID already in included set (line ~437)

### Filter Match Highlighting

When user types "home", the text "home" in both ID and name fields gets yellow background via `highlight_match()` function. This uses case-insensitive substring matching and splits text into Span segments.

### Tree State Management

The tui-tree-widget's `TreeState<String>` uses unique IDs as identifiers. When switching datasources or changing filters:
1. Reset TreeState to default
2. Select first item
3. If filter active, call `expand_filtered_nodes()` to open all relevant nodes

### TSV Data Format

All three TSV files follow same structure:
- Unique ID (string)
- Parent ID (optional)
- Name
- Tier 1..N (variable by dataset)
- Extension (optional, only Content/Audience)

Data loaded via `csv` crate with tab delimiter and serde deserialization.

## Dependencies

- **ratatui 0.30.0-beta.0**: TUI framework (upgraded from 0.29, modern APIs)
- **tui-tree-widget**: Local path dependency (`../tui-rs-tree-widget`) - modified for ratatui 0.30 compatibility
- **crossterm 0.29**: Terminal manipulation
- **csv 1.3**: TSV parsing with serde
- **anyhow 1**: Error handling

## Known Data Issues

- `product-2.0.tsv` contains 2 self-referencing entries (IDs 1000, 1037)
- These are handled gracefully by treating them as root nodes
- No circular reference chains exist in any dataset (verified)
