use honggfuzz::fuzz;
use fdg_sim::json;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            let json_src: &str = std::str::from_utf8(data).unwrap();
            let _ = json::graph_from_json(json_src);
        });
    }
}