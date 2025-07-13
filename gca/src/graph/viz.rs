use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;

/// Convert graph to DOT string
pub fn to_dot<N: std::fmt::Debug, E: std::fmt::Debug>(g: &DiGraph<N, E>) -> String {
    format!("{:?}", Dot::with_config(g, &[Config::EdgeNoLabel]))
}

/// Save graph as PNG using external `dot` binary
pub fn save_png(dot: &str, path: &str) -> anyhow::Result<()> {
    
    let dot_path = format!("{}.dot", path);
    std::fs::write(&dot_path, dot)?;
    let status = std::process::Command::new("dot")
        .args(["-Tpng", &dot_path, "-o", path])
        .status()?;
    if !status.success() {
        anyhow::bail!("graphviz dot failed");
    }
    Ok(())
}
