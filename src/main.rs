use anyhow::*;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, Tabs},
    DefaultTerminal,
};
use serde::{Deserialize, Serialize};

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
    selected_index: usize,
    scroll_offset: usize,
    scrollbar_state: ScrollbarState,
    show_popup: bool,
    popup_content: Vec<(String, String)>,
}

impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            datasource: Datasource::Product,
            filter_input: String::new(),
            products: load_products()?,
            content: load_content()?,
            audience: load_audience()?,
            selected_index: 0,
            scroll_offset: 0,
            scrollbar_state: ScrollbarState::default(),
            show_popup: false,
            popup_content: Vec::new(),
        })
    }

    fn switch_datasource(&mut self, datasource: Datasource) {
        self.datasource = datasource;
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.scrollbar_state = ScrollbarState::default();
    }

    fn filtered_items(&self) -> Vec<Vec<String>> {
        let filter_lower = self.filter_input.to_lowercase();

        match self.datasource {
            Datasource::Product => self
                .products
                .iter()
                .filter(|item| self.matches_all_fields(item, &filter_lower))
                .map(|item| self.format_item_as_row(item))
                .collect(),
            Datasource::Content => self
                .content
                .iter()
                .filter(|item| self.matches_all_fields(item, &filter_lower))
                .map(|item| self.format_item_as_row(item))
                .collect(),
            Datasource::Audience => self
                .audience
                .iter()
                .filter(|item| self.matches_all_fields(item, &filter_lower))
                .map(|item| self.format_item_as_row(item))
                .collect(),
        }
    }

    fn matches_all_fields<T: TaxonomyItem + ?Sized>(&self, item: &T, filter_lower: &str) -> bool {
        if filter_lower.is_empty() {
            return true;
        }

        // Search in unique_id (starts with)
        if item.unique_id().to_lowercase().starts_with(filter_lower) {
            return true;
        }

        // Search in parent (starts with)
        if let Some(parent) = item.parent() {
            if parent.to_lowercase().starts_with(filter_lower) {
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

    fn format_item_as_row<T: TaxonomyItem>(&self, item: &T) -> Vec<String> {
        let tiers = item.tiers().join(" > ");
        let parent = item.parent().unwrap_or("").to_string();
        let ext = item.extension().unwrap_or("").to_string();

        vec![
            item.unique_id().to_string(),
            parent,
            item.name().to_string(),
            tiers,
            ext,
        ]
    }

    fn column_headers(&self) -> Vec<&str> {
        match self.datasource {
            Datasource::Product => vec!["ID", "Parent", "Name", "Tiers", ""],
            Datasource::Content => vec!["ID", "Parent", "Name", "Tiers", "Ext"],
            Datasource::Audience => vec!["ID", "Parent", "Name", "Tiers", "Ext"],
        }
    }

    fn column_widths(&self) -> Vec<Constraint> {
        vec![
            Constraint::Length(10),  // ID
            Constraint::Length(10),  // Parent
            Constraint::Min(20),     // Name (flexible)
            Constraint::Min(20),     // Tiers (flexible)
            Constraint::Length(10),  // Extension
        ]
    }

    fn show_item_details(&mut self) {
        let filter_lower = self.filter_input.to_lowercase();

        let details = match self.datasource {
            Datasource::Product => {
                let filtered: Vec<&Product> = self.products
                    .iter()
                    .filter(|item| self.matches_all_fields(*item, &filter_lower))
                    .collect();

                if let Some(item) = filtered.get(self.selected_index) {
                    self.format_item_details(item)
                } else {
                    return;
                }
            }
            Datasource::Content => {
                let filtered: Vec<&Content> = self.content
                    .iter()
                    .filter(|item| self.matches_all_fields(*item, &filter_lower))
                    .collect();

                if let Some(item) = filtered.get(self.selected_index) {
                    self.format_item_details(item)
                } else {
                    return;
                }
            }
            Datasource::Audience => {
                let filtered: Vec<&Audience> = self.audience
                    .iter()
                    .filter(|item| self.matches_all_fields(*item, &filter_lower))
                    .collect();

                if let Some(item) = filtered.get(self.selected_index) {
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

    fn update_scroll(&mut self, viewport_height: usize) {
        // Ensure selected item is visible within viewport
        if self.selected_index < self.scroll_offset {
            // Scrolling up
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + viewport_height {
            // Scrolling down
            self.scroll_offset = self.selected_index.saturating_sub(viewport_height - 1);
        }

        // Sync scrollbar state
        let total_items = self.filtered_items().len();
        self.scrollbar_state = self.scrollbar_state
            .content_length(total_items)
            .viewport_content_length(viewport_height)
            .position(self.scroll_offset);
    }

    fn handle_key(&mut self, key: KeyEvent, viewport_height: usize) -> bool {
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
            KeyCode::Char(c) => {
                self.filter_input.push(c);
                self.selected_index = 0;
                self.scroll_offset = 0;
                self.scrollbar_state = ScrollbarState::default();
            }
            KeyCode::Backspace => {
                self.filter_input.pop();
                self.selected_index = 0;
                self.scroll_offset = 0;
                self.scrollbar_state = ScrollbarState::default();
            }
            KeyCode::Down => {
                let item_count = self.filtered_items().len();
                if item_count > 0 && self.selected_index < item_count - 1 {
                    self.selected_index += 1;
                    self.update_scroll(viewport_height);
                }
            }
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    self.update_scroll(viewport_height);
                }
            }
            KeyCode::PageDown => {
                let item_count = self.filtered_items().len();
                if item_count > 0 {
                    self.selected_index = (self.selected_index + 10).min(item_count - 1);
                    self.update_scroll(viewport_height);
                }
            }
            KeyCode::PageUp => {
                self.selected_index = self.selected_index.saturating_sub(10);
                self.update_scroll(viewport_height);
            }
            _ => {}
        }
        true
    }
}

// TUI rendering
fn ui(frame: &mut Frame, app: &App) {
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

    // Table of filtered items
    let filtered = app.filtered_items();
    let total_count = filtered.len();

    let headers = app.column_headers();
    let header_cells: Vec<Cell> = headers
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(app.datasource.color()).bold()))
        .collect();
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1);

    // Calculate viewport height (table area minus borders and header)
    let table_height = chunks[2].height.saturating_sub(3); // 2 for borders, 1 for header
    let viewport_end = (app.scroll_offset + table_height as usize).min(total_count);

    // Only show rows within the viewport
    let rows: Vec<Row> = filtered
        .into_iter()
        .enumerate()
        .skip(app.scroll_offset)
        .take(table_height as usize)
        .map(|(i, row_data)| {
            let cells: Vec<Cell> = row_data.into_iter().map(|c| Cell::from(c)).collect();
            let style = if i == app.selected_index {
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Blue)
            } else {
                Style::default().fg(Color::Gray)
            };
            Row::new(cells).style(style).height(1)
        }).collect();

    let title = if total_count == 0 {
        "Results (0 items)".to_string()
    } else {
        format!("Results ({} items, showing {}-{})", total_count, app.scroll_offset + 1, viewport_end)
    };

    let table = Table::new(rows, app.column_widths())
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title),
        )
        .column_spacing(1);

    frame.render_widget(table, chunks[2]);

    // Render scrollbar
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
        .thumb_symbol("█")
        .track_symbol(Some("│"))
        .thumb_style(Style::default().fg(app.datasource.color()))
        .track_style(Style::default().fg(Color::DarkGray));

    let mut scrollbar_state = ScrollbarState::default()
        .content_length(total_count)
        .viewport_content_length(table_height as usize)
        .position(app.scroll_offset);
    frame.render_stateful_widget(scrollbar, chunks[2], &mut scrollbar_state);

    // Help bar
    let help_text = if app.show_popup {
        "ESC/Enter: Close | q: Quit"
    } else {
        "Tab/Shift+Tab: Switch datasource | ↑↓: Navigate | Enter: View details | ESC/q: Quit"
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
        terminal.draw(|frame| ui(frame, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // Calculate viewport height: terminal height minus header(3), filter(3), help(1), borders
                    let terminal_height = terminal.size()?.height;
                    let viewport_height = terminal_height.saturating_sub(7 + 3) as usize; // 7 for UI chrome, 3 for table borders/header

                    if !app.handle_key(key, viewport_height) {
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
