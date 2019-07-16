#![feature(drain_filter)]
mod week1;

fn main() -> std::io::Result<()>{
    let g = week1::create_graph_from_file("SCC.txt")?;
    //println!("{}",g.len());
    //println!("{:?}",g);
    week1::dfs_loop(&g);
    Ok(())
   //let mut g = HashMap::new();
   //week1::dfs(&g, &0)
}
