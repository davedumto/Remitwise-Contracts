#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, Env, String};

#[test]
fn test_create_goal() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SavingsGoals);
    let client = SavingsGoalsClient::new(&env, &contract_id);
    
    let name = String::from_str(&env, "Education");
    let goal_id = client.create_goal(&name, &100000i128, &1735689600u64); // Target: $1000, date: 2025-01-01
    
    assert_eq!(goal_id, 1);
}

#[test]
fn test_add_to_goal() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SavingsGoals);
    let client = SavingsGoalsClient::new(&env, &contract_id);
    
    let name = String::from_str(&env, "Education");
    let goal_id = client.create_goal(&name, &100000i128, &1735689600u64);
    
    let current = client.add_to_goal(&goal_id, &10000i128);
    assert_eq!(current, 10000);
    
    let current = client.add_to_goal(&goal_id, &5000i128);
    assert_eq!(current, 15000);
}

#[test]
fn test_is_goal_completed() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SavingsGoals);
    let client = SavingsGoalsClient::new(&env, &contract_id);
    
    let name = String::from_str(&env, "Education");
    let goal_id = client.create_goal(&name, &100000i128, &1735689600u64);
    
    assert!(!client.is_goal_completed(&goal_id));
    
    client.add_to_goal(&goal_id, &100000i128);
    assert!(client.is_goal_completed(&goal_id));
}

