use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
fn main() {
    let conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .finish()
            .unwrap(),
    );
}
