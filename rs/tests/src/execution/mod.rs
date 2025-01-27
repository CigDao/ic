pub mod api_tests;
pub mod big_stable_memory;
pub mod call_on_cleanup;
pub mod canister_heartbeat;
pub mod canister_lifecycle;
pub mod cycles_transfer;
pub mod ingress_rate_limiting;
pub mod inter_canister_queries;
pub mod malicious_input;
pub mod nns_shielding;
pub mod queries;
pub mod registry_authentication_test;
pub mod request_signature_test;
pub mod system_api_security_test;
pub mod upgraded_pots;

use crate::driver::{
    ic::{InternetComputer, Subnet},
    test_env::TestEnv,
};
use ic_fondue::ic_instance::{LegacyInternetComputer, Subnet as LegacySubnet};
use ic_registry_subnet_type::SubnetType;

pub fn config_system_verified_application_subnets(env: TestEnv) {
    InternetComputer::new()
        .add_subnet(Subnet::fast_single_node(SubnetType::System))
        .add_subnet(Subnet::fast_single_node(SubnetType::VerifiedApplication))
        .add_subnet(Subnet::fast_single_node(SubnetType::Application))
        .setup_and_start(&env)
        .expect("failed to setup IC under test");
}

pub fn legacy_config_system_verified_application_subnets() -> LegacyInternetComputer {
    LegacyInternetComputer::new()
        .add_subnet(LegacySubnet::fast_single_node(SubnetType::System))
        .add_subnet(LegacySubnet::fast_single_node(
            SubnetType::VerifiedApplication,
        ))
        .add_subnet(LegacySubnet::fast_single_node(SubnetType::Application))
}

pub fn config_system_verified_subnets(env: TestEnv) {
    InternetComputer::new()
        .add_subnet(Subnet::fast_single_node(SubnetType::System))
        .add_subnet(Subnet::fast_single_node(SubnetType::VerifiedApplication))
        .setup_and_start(&env)
        .expect("failed to setup IC under test");
}

pub fn legacy_config_system_verified_subnets() -> LegacyInternetComputer {
    LegacyInternetComputer::new()
        .add_subnet(LegacySubnet::fast_single_node(SubnetType::System))
        .add_subnet(LegacySubnet::fast_single_node(
            SubnetType::VerifiedApplication,
        ))
}

pub fn config_many_system_subnets() -> InternetComputer {
    InternetComputer::new()
        .add_subnet(Subnet::fast_single_node(SubnetType::System))
        .add_subnet(Subnet::fast_single_node(SubnetType::VerifiedApplication))
        .add_subnet(Subnet::fast_single_node(SubnetType::Application))
        .add_subnet(Subnet::fast_single_node(SubnetType::System))
}

pub fn legacy_config_many_system_subnets() -> LegacyInternetComputer {
    LegacyInternetComputer::new()
        .add_subnet(LegacySubnet::fast_single_node(SubnetType::System))
        .add_subnet(LegacySubnet::fast_single_node(
            SubnetType::VerifiedApplication,
        ))
        .add_subnet(LegacySubnet::fast_single_node(SubnetType::Application))
        .add_subnet(LegacySubnet::fast_single_node(SubnetType::System))
}

// A special configuration for testing memory capacity limits.
pub fn legacy_config_memory_capacity() -> LegacyInternetComputer {
    LegacyInternetComputer::new().add_subnet(
        LegacySubnet::fast_single_node(SubnetType::System)
            // A tiny memory capacity
            .with_memory_capacity(20 * 1024 * 1024 /* 20 MiB */),
    )
}

// A special configuration for testing the maximum number of canisters on a
// subnet. The value is set to 3 for the tests.
pub fn config_max_number_of_canisters(env: TestEnv) {
    InternetComputer::new()
        .add_subnet(Subnet::fast_single_node(SubnetType::System).with_max_number_of_canisters(3))
        .setup_and_start(&env)
        .expect("failed to setup IC under test");
}
