#![feature(drain_filter)]
mod week1;

fn main() -> std::io::Result<()>{
    let g = week1::create_graph_from_file("SCC.txt")?;
    week1::dfs_loop(&g);
    Ok(())
}
