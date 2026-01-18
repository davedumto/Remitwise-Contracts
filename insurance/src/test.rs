#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env, String};

#[test]
fn test_create_policy() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Insurance);
    let client = InsuranceClient::new(&env, &contract_id);
    
    let name = String::from_str(&env, "Health Insurance");
    let coverage_type = String::from_str(&env, "health");
    let policy_id = client.create_policy(
        &name,
        &coverage_type,
        &2000i128, // $20 monthly premium
        &100000i128, // $1000 coverage
    );
    
    assert_eq!(policy_id, 1);
}

#[test]
fn test_pay_premium() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Insurance);
    let client = InsuranceClient::new(&env, &contract_id);
    
    let name = String::from_str(&env, "Health Insurance");
    let coverage_type = String::from_str(&env, "health");
    let policy_id = client.create_policy(&name, &coverage_type, &2000i128, &100000i128);
    
    let result = client.pay_premium(&policy_id);
    assert!(result);
    
    let policy = client.get_policy(&policy_id).unwrap();
    assert!(policy.active);
}

#[test]
fn test_get_total_monthly_premium() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Insurance);
    let client = InsuranceClient::new(&env, &contract_id);
    
    let name1 = String::from_str(&env, "Health Insurance");
    let name2 = String::from_str(&env, "Emergency Insurance");
    let coverage_type = String::from_str(&env, "health");
    
    client.create_policy(&name1, &coverage_type, &2000i128, &100000i128);
    client.create_policy(&name2, &coverage_type, &1500i128, &50000i128);
    
    let total = client.get_total_monthly_premium();
    assert_eq!(total, 3500);
}

