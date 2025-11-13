use anyhow::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Tabs},
    DefaultTerminal,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tui_tree_widget::{Tree, TreeItem, TreeState};

const PRODUCT_TSV: &str = include_str!("../product-2.0.tsv");
const CONTENT_TSV: &str = include_str!("../content-3.1.tsv");
const AUDIENCE_TSV: &str = include_str!("../audience-1.1.tsv");

// Data structures
trait TaxonomyItem {
    fn unique_id(&self) -> &str;
    fn parent(&self) -> Option<&str>;
    fn name(&self) -> &str;
    fn tiers(&self) -> Vec<&str>;
    fn extension(&self) -> Option<&str>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "Unique ID")]
    unique_id: String,
    #[serde(rename = "Parent")]
    parent: Option<String>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Tier 1")]
    tier_1: Option<String>,
    #[serde(rename = "Tier 2")]
    tier_2: Option<String>,
    #[serde(rename = "Tier 3")]
    tier_3: Option<String>,
    #[serde(rename = "Tier 4")]
    tier_4: Option<String>,
    #[serde(rename = "Extension")]
    ext: Option<String>,
}

impl TaxonomyItem for Content {
    fn unique_id(&self) -> &str {
        &self.unique_id
    }
    fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn tiers(&self) -> Vec<&str> {
        [
            self.tier_1.as_deref(),
            self.tier_2.as_deref(),
            self.tier_3.as_deref(),
            self.tier_4.as_deref(),
        ]
        .iter()
        .filter_map(|&t| t.filter(|s| !s.is_empty()))
        .collect()
    }
    fn extension(&self) -> Option<&str> {
        self.ext.as_deref()
    }
}

impl TaxonomyItem for &Content {
    fn unique_id(&self) -> &str {
        (*self).unique_id()
    }
    fn parent(&self) -> Option<&str> {
        (*self).parent()
    }
    fn name(&self) -> &str {
        (*self).name()
    }
    fn tiers(&self) -> Vec<&str> {
        (*self).tiers()
    }
    fn extension(&self) -> Option<&str> {
        (*self).extension()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Product {
    #[serde(rename = "Unique ID")]
    unique_id: String,
    #[serde(rename = "Parent ID")]
    parent: Option<String>,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Tier 1")]
    tier_1: Option<String>,
    #[serde(rename = "Tier 2")]
    tier_2: Option<String>,
    #[serde(rename = "Tier 3")]
    tier_3: Option<String>,
}

impl TaxonomyItem for Product {
    fn unique_id(&self) -> &str {
        &self.unique_id
    }
    fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn tiers(&self) -> Vec<&str> {
        [
            self.tier_1.as_deref(),
            self.tier_2.as_deref(),
            self.tier_3.as_deref(),
        ]
        .iter()
        .filter_map(|&t| t.filter(|s| !s.is_empty()))
        .collect()
    }
    fn extension(&self) -> Option<&str> {
        None
    }
}

impl TaxonomyItem for &Product {
    fn unique_id(&self) -> &str {
        (*self).unique_id()
    }
    fn parent(&self) -> Option<&str> {
        (*self).parent()
    }
    fn name(&self) -> &str {
        (*self).name()
    }
    fn tiers(&self) -> Vec<&str> {
        (*self).tiers()
    }
    fn extension(&self) -> Option<&str> {
        (*self).extension()
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Audience {
    #[serde(rename = "Unique ID")]
    unique_id: String,
    #[serde(rename = "Parent ID")]
    parent: Option<String>,
    #[serde(rename = "Condensed Name (1st, 2nd, Last Tier)")]
    name: String,
    #[serde(rename = "Tier 1")]
    tier_1: Option<String>,
    #[serde(rename = "Tier 2")]
    tier_2: Option<String>,
    #[serde(rename = "Tier 3")]
    tier_3: Option<String>,
    #[serde(rename = "Tier 4")]
    tier_4: Option<String>,
    #[serde(rename = "Tier 5")]
    tier_5: Option<String>,
    #[serde(rename = "Tier 6")]
    tier_6: Option<String>,
    #[serde(rename = "*Extension Notes")]
    ext: Option<String>,
}

impl TaxonomyItem for Audience {
    fn unique_id(&self) -> &str {
        &self.unique_id
    }
    fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn tiers(&self) -> Vec<&str> {
        [
            self.tier_1.as_deref(),
            self.tier_2.as_deref(),
            self.tier_3.as_deref(),
            self.tier_4.as_deref(),
            self.tier_5.as_deref(),
            self.tier_6.as_deref(),
        ]
        .iter()
        .filter_map(|&t| t.filter(|s| !s.is_empty()))
        .collect()
    }
    fn extension(&self) -> Option<&str> {
        self.ext.as_deref()
    }
}

impl TaxonomyItem for &Audience {
    fn unique_id(&self) -> &str {
        (*self).unique_id()
    }
    fn parent(&self) -> Option<&str> {
        (*self).parent()
    }
    fn name(&self) -> &str {
        (*self).name()
    }
    fn tiers(&self) -> Vec<&str> {
        (*self).tiers()
    }
    fn extension(&self) -> Option<&str> {
        (*self).extension()
    }
}

// Datasource enum
#[derive(Debug, Clone, Copy, PartialEq)]
enum Datasource {
    Product,
    Content,
    Audience,
}

impl Datasource {
    fn next(self) -> Self {
        match self {
            Datasource::Product => Datasource::Content,
            Datasource::Content => Datasource::Audience,
            Datasource::Audience => Datasource::Product,
        }
    }

    fn previous(self) -> Self {
        match self {
            Datasource::Product => Datasource::Audience,
            Datasource::Content => Datasource::Product,
            Datasource::Audience => Datasource::Content,
        }
    }

    fn color(self) -> Color {
        match self {
            Datasource::Product => Color::Yellow,
            Datasource::Content => Color::Cyan,
            Datasource::Audience => Color::Red,
        }
    }

    fn bright_color(self) -> Color {
        match self {
            Datasource::Product => Color::LightYellow,
            Datasource::Content => Color::LightCyan,
            Datasource::Audience => Color::LightRed,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Datasource::Product => "Product",
            Datasource::Content => "Content",
            Datasource::Audience => "Audience",
        }
    }

    fn index(self) -> usize {
        match self {
            Datasource::Product => 0,
            Datasource::Content => 1,
            Datasource::Audience => 2,
        }
    }
}

// Data loading functions
fn load_products() -> Result<Vec<Product>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(PRODUCT_TSV.as_bytes());

    let mut items = Vec::new();
    for result in reader.deserialize() {
        items.push(result?);
    }

    Ok(items)
}

fn load_content() -> Result<Vec<Content>> {
    let mut lines = CONTENT_TSV.lines();
    // Skip first line (section header)
    lines.next();

    // Keep second line (actual column headers) and all data lines
    let remaining_content = lines.collect::<Vec<_>>().join("\n");

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(remaining_content.as_bytes());

    let mut items = Vec::new();
    for result in reader.deserialize() {
        items.push(result?);
    }

    Ok(items)
}

fn load_audience() -> Result<Vec<Audience>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(true)
        .from_reader(AUDIENCE_TSV.as_bytes());

    let mut items = Vec::new();
    for result in reader.deserialize() {
        items.push(result?);
    }

    Ok(items)
}

// App state
struct App {
    datasource: Datasource,
    filter_input: String,
    products: Vec<Product>,
    content: Vec<Content>,
    audience: Vec<Audience>,
    tree_state: TreeState<String>,
    show_popup: bool,
    popup_content: Vec<(String, String)>,
}

impl App {
    fn new() -> Result<Self> {
        let mut tree_state = TreeState::default();
        tree_state.select_first();

        Ok(Self {
            datasource: Datasource::Product,
            filter_input: String::new(),
            products: load_products()?,
            content: load_content()?,
            audience: load_audience()?,
            tree_state,
            show_popup: false,
            popup_content: Vec::new(),
        })
    }

    fn switch_datasource(&mut self, datasource: Datasource) {
        self.datasource = datasource;
        self.tree_state = TreeState::default();
        self.tree_state.select_first();
        if !self.filter_input.is_empty() {
            self.expand_filtered_nodes();
        }
    }

    fn filtered_tree_items(&self) -> Vec<TreeItem<'static, String>> {
        let filter_lower = self.filter_input.to_lowercase();

        // If no filter, build full tree
        if filter_lower.is_empty() {
            return match self.datasource {
                Datasource::Product => build_tree_items(&self.products, ""),
                Datasource::Content => build_tree_items(&self.content, ""),
                Datasource::Audience => build_tree_items(&self.audience, ""),
            };
        }

        // Filter items and build tree with full path + descendants
        match self.datasource {
            Datasource::Product => self.filtered_tree_from_items(&self.products, &filter_lower),
            Datasource::Content => self.filtered_tree_from_items(&self.content, &filter_lower),
            Datasource::Audience => self.filtered_tree_from_items(&self.audience, &filter_lower),
        }
    }

    fn filtered_tree_from_items<T: TaxonomyItem + Clone>(&self, items: &[T], filter_lower: &str) -> Vec<TreeItem<'static, String>> {
        // Find all matching items
        let matching_ids: HashSet<String> = items
            .iter()
            .filter(|item| self.matches_all_fields(*item, filter_lower))
            .map(|item| item.unique_id().to_string())
            .collect();

        if matching_ids.is_empty() {
            return vec![];
        }

        // Build parent map for ancestor lookup
        let parent_map: HashMap<String, Option<String>> = items
            .iter()
            .map(|item| (item.unique_id().to_string(), item.parent().map(|s| s.to_string())))
            .collect();

        // Collect all IDs to include: matches + all ancestors + all descendants
        let mut included_ids: HashSet<String> = HashSet::new();

        // Add matches
        included_ids.extend(matching_ids.iter().cloned());

        // Add all ancestors of matches
        for match_id in &matching_ids {
            let mut current_id = match_id.clone();
            let mut visited = HashSet::new();
            while let Some(Some(parent_id)) = parent_map.get(&current_id) {
                // Prevent infinite loop on circular references
                if visited.contains(&current_id) {
                    break;
                }
                visited.insert(current_id.clone());
                included_ids.insert(parent_id.clone());
                current_id = parent_id.clone();
            }
        }

        // Add all descendants of matches
        for match_id in &matching_ids {
            self.add_all_descendants(match_id, items, &mut included_ids);
        }

        // Filter items to only included IDs
        let filtered_items: Vec<T> = items
            .iter()
            .filter(|item| included_ids.contains(item.unique_id()))
            .cloned()
            .collect();

        // Build tree from filtered items
        build_tree_items(&filtered_items, filter_lower)
    }

    fn add_all_descendants<T: TaxonomyItem>(&self, parent_id: &str, items: &[T], included_ids: &mut HashSet<String>) {
        for item in items {
            if let Some(item_parent) = item.parent() {
                if item_parent == parent_id {
                    let child_id = item.unique_id().to_string();
                    // Prevent infinite recursion on circular references
                    if !included_ids.contains(&child_id) {
                        included_ids.insert(child_id.clone());
                        self.add_all_descendants(&child_id, items, included_ids);
                    }
                }
            }
        }
    }

    fn expand_filtered_nodes(&mut self) {
        if !self.filter_input.is_empty() {
            let tree_items = self.filtered_tree_items();
            let all_paths = collect_all_tree_paths(&tree_items, vec![]);
            for path in all_paths {
                self.tree_state.open(path);
            }
        }
    }

    fn matches_all_fields<T: TaxonomyItem + ?Sized>(&self, item: &T, filter_lower: &str) -> bool {
        if filter_lower.is_empty() {
            return true;
        }

        // Search in unique_id (exact match)
        if item.unique_id().to_lowercase() == filter_lower {
            return true;
        }

        // Search in parent (exact match)
        if let Some(parent) = item.parent() {
            if parent.to_lowercase() == filter_lower {
                return true;
            }
        }

        // Search in name
        if item.name().to_lowercase().contains(filter_lower) {
            return true;
        }

        // Search in tiers
        for tier in item.tiers() {
            if tier.to_lowercase().contains(filter_lower) {
                return true;
            }
        }

        // Search in extension
        if let Some(ext) = item.extension() {
            if ext.to_lowercase().contains(filter_lower) {
                return true;
            }
        }

        false
    }

    fn show_item_details(&mut self) {
        // Get the selected item's unique ID from the tree state
        let selected_path = self.tree_state.selected();
        let selected_id = match selected_path.last() {
            Some(id) => id,
            None => return,
        };

        let details = match self.datasource {
            Datasource::Product => {
                let item = self.products
                    .iter()
                    .find(|item| item.unique_id() == selected_id);

                if let Some(item) = item {
                    self.format_item_details(item)
                } else {
                    return;
                }
            }
            Datasource::Content => {
                let item = self.content
                    .iter()
                    .find(|item| item.unique_id() == selected_id);

                if let Some(item) = item {
                    self.format_item_details(item)
                } else {
                    return;
                }
            }
            Datasource::Audience => {
                let item = self.audience
                    .iter()
                    .find(|item| item.unique_id() == selected_id);

                if let Some(item) = item {
                    self.format_item_details(item)
                } else {
                    return;
                }
            }
        };

        self.popup_content = details;
        self.show_popup = true;
    }

    fn format_item_details<T: TaxonomyItem>(&self, item: &T) -> Vec<(String, String)> {
        let mut details = vec![
            ("Unique ID".to_string(), item.unique_id().to_string()),
            ("Parent ID".to_string(), item.parent().unwrap_or("").to_string()),
            ("Name".to_string(), item.name().to_string()),
        ];

        let tiers = item.tiers();
        for (i, tier) in tiers.iter().enumerate() {
            details.push((format!("Tier {}", i + 1), tier.to_string()));
        }

        if let Some(ext) = item.extension() {
            if !ext.is_empty() {
                details.push(("Extension".to_string(), ext.to_string()));
            }
        }

        details
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Handle popup-specific keys first
        if self.show_popup {
            match key.code {
                KeyCode::Esc | KeyCode::Enter | KeyCode::Char('q') => {
                    self.show_popup = false;
                    return key.code != KeyCode::Char('q');
                }
                _ => return true,
            }
        }

        // Handle normal navigation
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return false,
            KeyCode::Enter => {
                self.show_item_details();
            }
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    self.switch_datasource(self.datasource.previous());
                } else {
                    self.switch_datasource(self.datasource.next());
                }
            }
            KeyCode::Char(' ') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.tree_state.toggle_selected();
            }
            KeyCode::Char(c) => {
                self.filter_input.push(c);
                self.tree_state = TreeState::default();
                self.tree_state.select_first();
                self.expand_filtered_nodes();
            }
            KeyCode::Backspace => {
                self.filter_input.pop();
                self.tree_state = TreeState::default();
                self.tree_state.select_first();
                self.expand_filtered_nodes();
            }
            KeyCode::Down => {
                self.tree_state.key_down();
            }
            KeyCode::Up => {
                self.tree_state.key_up();
            }
            KeyCode::Left => {
                self.tree_state.key_left();
            }
            KeyCode::Right => {
                self.tree_state.key_right();
            }
            KeyCode::PageDown => {
                for _ in 0..10 {
                    self.tree_state.key_down();
                }
            }
            KeyCode::PageUp => {
                for _ in 0..10 {
                    self.tree_state.key_up();
                }
            }
            _ => {}
        }
        true
    }
}

// Tree building helpers
fn build_tree_items<T: TaxonomyItem>(items: &[T], filter: &str) -> Vec<TreeItem<'static, String>> {
    let mut children_map: HashMap<Option<String>, Vec<&T>> = HashMap::new();

    // Group items by parent
    for item in items {
        // Treat self-references as root nodes
        let parent_key = match item.parent() {
            Some(p) if p == item.unique_id() => None,
            Some(p) => Some(p.to_string()),
            None => None,
        };
        children_map.entry(parent_key).or_default().push(item);
    }

    // Build tree starting from root nodes (no parent)
    build_tree_recursive(&children_map, None, filter)
}

fn build_tree_recursive<'a, T: TaxonomyItem>(
    children_map: &HashMap<Option<String>, Vec<&'a T>>,
    parent_id: Option<String>,
    filter: &str,
) -> Vec<TreeItem<'static, String>> {
    let children = match children_map.get(&parent_id) {
        Some(children) => children,
        None => return vec![],
    };

    children.iter().map(|item| {
        let id = item.unique_id().to_string();
        let name = item.name().to_string();
        let node_children = build_tree_recursive(children_map, Some(id.clone()), filter);

        // Format: [bold ID] name with highlighted matches
        let mut display_spans = Vec::new();
        // Add highlighted ID spans with bold style
        for span in highlight_match(&id, filter) {
            display_spans.push(Span::styled(span.content.to_string(), span.style.bold()));
        }
        display_spans.push(Span::raw(" "));
        // Add highlighted name spans
        display_spans.extend(highlight_match(&name, filter));
        let display_text = Line::from(display_spans);

        TreeItem::new(id.clone(), display_text, node_children)
            .expect("Failed to create tree item")
    }).collect()
}

fn count_tree_items(items: &[TreeItem<String>]) -> usize {
    items.iter().map(|item| {
        1 + count_tree_items(item.children())
    }).sum()
}

fn collect_all_tree_paths(items: &[TreeItem<String>], current_path: Vec<String>) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    for item in items {
        let mut path = current_path.clone();
        path.push(item.identifier().clone());
        paths.push(path.clone());
        // Recursively collect child paths
        paths.extend(collect_all_tree_paths(item.children(), path));
    }
    paths
}

fn highlight_match(text: &str, filter: &str) -> Vec<Span<'static>> {
    if filter.is_empty() {
        return vec![Span::raw(text.to_string())];
    }

    let text_lower = text.to_lowercase();
    let filter_lower = filter.to_lowercase();

    // Find match position
    if let Some(pos) = text_lower.find(&filter_lower) {
        let mut spans = Vec::new();
        if pos > 0 {
            spans.push(Span::raw(text[..pos].to_string()));
        }
        let end = pos + filter.len();
        spans.push(Span::styled(
            text[pos..end].to_string(),
            Style::default().fg(Color::Black).bg(Color::Yellow)
        ));
        if end < text.len() {
            spans.push(Span::raw(text[end..].to_string()));
        }
        spans
    } else {
        vec![Span::raw(text.to_string())]
    }
}

fn calculate_flat_index(
    items: &[TreeItem<String>],
    tree_state: &TreeState<String>,
    current_path: Vec<String>,
) -> Option<usize> {
    let selected = tree_state.selected();
    let opened = tree_state.opened();
    let mut index = 0;

    for item in items {
        let mut item_path = current_path.clone();
        item_path.push(item.identifier().clone());

        // Check if this is the selected item
        if item_path == selected {
            return Some(index);
        }

        index += 1;

        // If this node is opened, recursively check children
        if opened.contains(&item_path) {
            if let Some(child_index) = calculate_flat_index(item.children(), tree_state, item_path) {
                return Some(index + child_index);
            }
            // Count all visible children
            index += count_visible_items(item.children(), opened, &current_path, item.identifier());
        }
    }

    None
}

fn count_visible_items(
    items: &[TreeItem<String>],
    opened: &HashSet<Vec<String>>,
    parent_path: &[String],
    current_id: &str,
) -> usize {
    let mut count = 0;
    for item in items {
        count += 1; // Count this item

        let mut item_path = parent_path.to_vec();
        item_path.push(current_id.to_string());
        item_path.push(item.identifier().clone());

        // If opened, count children too
        if opened.contains(&item_path) {
            count += count_visible_items(item.children(), opened, &item_path[..item_path.len()-1], item.identifier());
        }
    }
    count
}

fn count_visible_tree_items(
    items: &[TreeItem<String>],
    tree_state: &TreeState<String>,
) -> usize {
    let opened = tree_state.opened();
    let mut count = 0;

    for item in items {
        count += 1; // Count this item

        let item_path = vec![item.identifier().clone()];

        // If opened, count children too
        if opened.contains(&item_path) {
            count += count_visible_items(item.children(), opened, &[], item.identifier());
        }
    }

    count
}

// TUI rendering
fn ui(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Split into sections: header, filter, list, help
    let layout = Layout::vertical([
        Constraint::Length(3), // Header with datasource tabs
        Constraint::Length(3), // Filter input
        Constraint::Min(0),     // List
        Constraint::Length(1),  // Help bar
    ]);
    let chunks: [Rect; 4] = area.layout(&layout);

    // Header with datasource tabs
    let tabs = Tabs::new(vec!["Product", "Content", "Audience"])
        .block(Block::default().borders(Borders::ALL).title("Datasource"))
        .select(app.datasource.index())
        .style(Style::default().fg(Color::Gray))
        .highlight_style(Style::default().fg(app.datasource.color()).bold())
        .divider("|");

    frame.render_widget(tabs, chunks[0]);

    // Filter input
    let filter_text = if app.filter_input.is_empty() {
        "Type to filter...".to_string()
    } else {
        app.filter_input.clone()
    };

    let filter = Paragraph::new(filter_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Filter"));

    frame.render_widget(filter, chunks[1]);

    // Tree of filtered items
    let tree_items = app.filtered_tree_items();
    let total_count = count_tree_items(&tree_items);

    let title = format!("Results ({} items)", total_count);

    let tree = Tree::new(&tree_items)
        .expect("Failed to create tree widget")
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .fg(app.datasource.bright_color())
                .bg(Color::Rgb(30, 30, 30))
                .bold()
        )
        .node_closed_symbol("▶ ")
        .node_open_symbol("▼ ")
        .node_no_children_symbol("  ");

    frame.render_stateful_widget(tree, chunks[2], &mut app.tree_state);

    // Render scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
        .thumb_symbol("█")
        .track_symbol(Some("│"))
        .thumb_style(Style::default().fg(app.datasource.color()))
        .track_style(Style::default().fg(Color::DarkGray));

    let viewport_height = chunks[2].height.saturating_sub(2) as usize; // Subtract borders
    let scroll_position = calculate_flat_index(&tree_items, &app.tree_state, vec![]).unwrap_or(0);
    let visible_count = count_visible_tree_items(&tree_items, &app.tree_state);

    let mut scrollbar_state = ScrollbarState::default()
        .content_length(visible_count)
        .viewport_content_length(viewport_height)
        .position(scroll_position);

    frame.render_stateful_widget(scrollbar, chunks[2], &mut scrollbar_state);

    // Help bar
    let help_text = if app.show_popup {
        "ESC/Enter: Close | q: Quit"
    } else {
        "Tab/Shift+Tab: Switch | ↑↓: Navigate | ←→: Collapse/Expand | Ctrl+Space: Toggle | Enter: Details | ESC/q: Quit"
    };
    let help = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));

    frame.render_widget(help, chunks[3]);

    // Render popup if active
    if app.show_popup {
        render_popup(frame, app);
    }
}

fn render_popup(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create centered popup (60% width, 80% height)
    let popup_area = Rect::centered(area, Constraint::Percentage(60), Constraint::Percentage(80));

    // Clear the background
    frame.render_widget(Clear, popup_area);

    // Render the popup block
    let block = Block::default()
        .title(format!(" {} Details ", app.datasource.name()))
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(app.datasource.color()));

    frame.render_widget(block, popup_area);

    // Render the content
    let inner_area = Rect {
        x: popup_area.x + 2,
        y: popup_area.y + 2,
        width: popup_area.width.saturating_sub(4),
        height: popup_area.height.saturating_sub(3),
    };

    let mut lines = Vec::new();
    for (label, value) in &app.popup_content {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{}: ", label),
                Style::default().fg(app.datasource.color()).bold(),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled(
                format!("  {}", value),
                Style::default().fg(Color::White),
            ),
        ]));
        lines.push(Line::from("")); // Empty line for spacing
    }

    let paragraph = Paragraph::new(lines)
        .style(Style::default().bg(Color::Black))
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(paragraph, inner_area);
}

fn run_app(terminal: &mut DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|frame| ui(frame, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if !app.handle_key(key) {
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    ratatui::run(|terminal| {
        let app = App::new()?;
        run_app(terminal, app)
    })
}
