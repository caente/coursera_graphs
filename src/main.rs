#![feature(drain_filter)]
mod week1;

fn main() -> std::io::Result<()> {
    let g = week1::create_graph_from_file("graph.txt")?;
    println!("graph created");
    //println!("g:{:?}",g);
    let g_rev = week1::reverse_graph(&g);
    println!("reversed graph created");
    //println!("g_rev:{:?}",g_rev);
    let week1::Finished {
        mut finishing_times,
        ..
    } = week1::dfs_loop(&g_rev);
    println!("dfs finished");
    let renamed_graph = week1::rename_graph(&g, &mut finishing_times);
    println!("renamed graph");
    //println!("renamed_graph:{:?}",renamed_graph);
    let week1::Finished { mut sizes, .. } = week1::dfs_loop(&renamed_graph);
    sizes.sort();
    sizes.reverse();
    sizes.truncate(5);
    println!("Sizes: {:?}", sizes);
    Ok(())
}
