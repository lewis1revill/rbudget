use rbudget::simulation::Simulation;

fn main() {
    let mut sim = Simulation::default();
    sim.load();
    for (values, date) in sim.iter().take(5) {
        println!("{}:", date);
        for kv in values {
            println!("Account {}, current value {}", kv.0.id_val, kv.1.to_string());
        }
    }
}
