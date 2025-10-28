use anyhow::*;
use clap::Parser;
use console::style;
use serde::{Deserialize, Serialize};
use streamwerk_csv::prelude::*;
use streamwerk_fs::prelude::*;

const PRODUCT_TSV: &str = include_str!("../product-2.0.tsv");
const CONTENT_TSV: &str = include_str!("../content-3.1.tsv");
const AUDIENCE_TSV: &str = include_str!("../audience-1.1.tsv");

#[derive(Parser, Debug)]
#[command(name = "iab")]
#[command(about = "IAB taxonomy pipeline processor", long_about = None)]
#[command(version = version_string())]
struct Cli {
    /// Process product pipeline
    #[arg(short, long)]
    product: bool,

    /// Process content pipeline
    #[arg(short, long)]
    content: bool,

    /// Process audience pipeline
    #[arg(short, long)]
    audience: bool,

    /// Filter by unique ID
    #[arg(short, long, conflicts_with_all = ["parent", "name"])]
    id: Option<String>,

    /// Filter by parent ID
    #[arg(short = 't', long, conflicts_with_all = ["id", "name"])]
    parent: Option<String>,

    /// Filter by name (case-insensitive substring match)
    #[arg(short, long, conflicts_with_all = ["id", "parent"])]
    name: Option<String>,
}

fn version_string() -> &'static str {
    concat!(
        env!("CARGO_PKG_VERSION"),
        "\nProduct: 2.0",
        "\nContent: 3.1",
        "\nAudience: 1.1",
        "\n",
        "\nhttps://github.com/InteractiveAdvertisingBureau/Taxonomies"
    )
}

trait TaxonomyItem {
    fn unique_id(&self) -> &str;
    fn parent(&self) -> Option<&str>;
    fn name(&self) -> &str;
}

#[derive(Clone, Copy)]
enum Filter<'a> {
    Id(&'a str),
    Parent(&'a str),
    Name(&'a str),
}

fn matches_filter<T: TaxonomyItem>(item: &T, filter: &Filter) -> bool {
    match filter {
        Filter::Id(id) => item.unique_id() == *id,
        Filter::Parent(parent_id) => {
            item.parent() == Some(parent_id) || item.unique_id() == *parent_id
        }
        Filter::Name(name) => {
            let item_name_lower = item.name().to_lowercase();
            let filter_lower = name.to_lowercase();
            item_name_lower.contains(&filter_lower)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    unique_id: String,
    parent: Option<String>,
    name: String,
    tier_1: Option<String>,
    tier_2: Option<String>,
    tier_3: Option<String>,
    tier_4: Option<String>,
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
}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tiers: Vec<&str> = [
            self.tier_1.as_deref(),
            self.tier_2.as_deref(),
            self.tier_3.as_deref(),
            self.tier_4.as_deref(),
        ]
        .iter()
        .filter_map(|&t| t.filter(|s| !s.is_empty()))
        .collect();

        write!(
            f,
            "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n",
            style("Unique ID").bold().cyan(),
            self.unique_id,
            style("Parent ID").bold().cyan(),
            self.parent.as_deref().unwrap_or_default(),
            style("Name").bold().cyan(),
            self.name,
            style("Tiers").bold().cyan(),
            tiers.join(" | "),
            style("Extension").bold().cyan(),
            self.ext.as_deref().unwrap_or_default()
        )
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Product {
    unique_id: String,
    parent: Option<String>,
    name: String,
    tier_1: Option<String>,
    tier_2: Option<String>,
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
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tiers: Vec<&str> = [
            self.tier_1.as_deref(),
            self.tier_2.as_deref(),
            self.tier_3.as_deref(),
        ]
        .iter()
        .filter_map(|&t| t.filter(|s| !s.is_empty()))
        .collect();

        write!(
            f,
            "{}: {}\n{}: {}\n{}: {}\n{}: {}\n",
            style("Unique ID").bold().yellow(),
            self.unique_id,
            style("Parent ID").bold().yellow(),
            self.parent.as_deref().unwrap_or_default(),
            style("Name").bold().yellow(),
            self.name,
            style("Tiers").bold().yellow(),
            tiers.join(" | "),
        )
    }
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Audience {
    unique_id: String,
    parent: Option<String>,
    name: String,
    tier_1: Option<String>,
    tier_2: Option<String>,
    tier_3: Option<String>,
    tier_4: Option<String>,
    tier_5: Option<String>,
    tier_6: Option<String>,
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
}

impl std::fmt::Display for Audience {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tiers: Vec<&str> = [
            self.tier_1.as_deref(),
            self.tier_2.as_deref(),
            self.tier_3.as_deref(),
            self.tier_4.as_deref(),
            self.tier_5.as_deref(),
            self.tier_6.as_deref(),
        ]
        .iter()
        .filter_map(|&t| t.filter(|s| !s.is_empty()))
        .collect();

        write!(
            f,
            "{}: {}\n{}: {}\n{}: {}\n{}: {}\n{}: {}\n",
            style("Unique ID").bold().red(),
            self.unique_id,
            style("Parent ID").bold().red(),
            self.parent.as_deref().unwrap_or_default(),
            style("Name").bold().red(),
            self.name,
            style("Tiers").bold().red(),
            tiers.join(" | "),
            style("Extension").bold().red(),
            self.ext.as_deref().unwrap_or_default()
        )
    }
}

fn extract_lines(input: &str) -> Result<impl Stream<Item = Result<String>> + Send> {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    Ok(iter_ok(lines))
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Determine pipeline
    let pipeline_count = [cli.product, cli.content, cli.audience]
        .iter()
        .filter(|&&x| x)
        .count();

    if pipeline_count == 0 {
        eprintln!("Error: Must specify one of --product, --content, or --audience");
        std::process::exit(1);
    }

    if pipeline_count > 1 {
        eprintln!("Error: Can only specify one pipeline at a time");
        std::process::exit(1);
    }

    // Determine filter
    let filter = if let Some(id) = &cli.id {
        Filter::Id(id)
    } else if let Some(parent) = &cli.parent {
        Filter::Parent(parent)
    } else if let Some(name) = &cli.name {
        Filter::Name(name)
    } else {
        eprintln!("Error: Must specify one of --id, --parent, or --name");
        std::process::exit(1);
    };

    if cli.product {
        let transform_product = CsvDeserializer::with_config(CsvConfig::tsv())
            .filter(move |product: &Product| matches_filter(product, &filter))
            .map(|product: Product| product.to_string());

        let pipeline_product = EtlPipeline::new(
            FnExtract(extract_lines).skip(1),
            transform_product,
            StdoutLoad,
        );
        pipeline_product.run(PRODUCT_TSV).await.unwrap();
    } else if cli.content {
        let transform_content = CsvDeserializer::with_config(CsvConfig::tsv())
            .filter(move |content: &Content| matches_filter(content, &filter))
            .map(|content: Content| content.to_string());

        let pipeline_content = EtlPipeline::new(
            FnExtract(extract_lines).skip(2),
            transform_content,
            StdoutLoad,
        );
        pipeline_content.run(CONTENT_TSV).await.unwrap();
    } else if cli.audience {
        let transform_audience = CsvDeserializer::with_config(CsvConfig::tsv())
            .filter(move |audience: &Audience| matches_filter(audience, &filter))
            .map(|audience: Audience| audience.to_string());

        let pipeline_audience = EtlPipeline::new(
            FnExtract(extract_lines).skip(1),
            transform_audience,
            StdoutLoad,
        );
        pipeline_audience.run(AUDIENCE_TSV).await.unwrap();
    }
}
