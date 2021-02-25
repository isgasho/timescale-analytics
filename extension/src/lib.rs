pub mod tdigest;
pub mod hyperloglog;
pub mod uddsketch;
pub mod time_weighted_average;

mod palloc;
mod aggregate_utils;
mod type_builder;
mod serialization;
mod schema_test;

pgx::pg_module_magic!();

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
