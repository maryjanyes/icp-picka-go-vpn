use ic_cdk::{update, query};
use std::cell::RefCell;
use std::vec::Vec;
use candid::{CandidType, Principal};
mod storage;
mod utils;

thread_local! {
    static VPN_VAULTS: RefCell<Vec<VpnVault>> = RefCell::new(Vec::<VpnVault>::new());
    static VPN_SERVERS_IN_USAGE: RefCell<Vec<VpnServerInUsage>> = RefCell::new(Vec::<VpnServerInUsage>::new());
}

#[derive(Clone, CandidType, serde::Deserialize)]
enum VpnRole {
    CUSTOMER,
    RENTER
}

#[derive(Clone, CandidType)]
struct VpnServerInUsage {
    server_ip: String,
    usage_by_clients: i64,
    max_clients: i64
}

#[derive(Clone, CandidType)]
struct VpnVault {
    vault_type: VpnRole,
    vault_dedicated_ip_servers: Vec<String>,
    vault_rented_ip_servers: Vec<String>,
    vault_owner: Principal,
    vault_credits: i64
}

#[query]
fn get_vault() -> VpnVault {
    VPN_VAULTS.with_borrow(|vaults| {
        let clo = move || vaults.iter().find(|v| v.vault_owner == ic_cdk::caller()).cloned();

        clo().unwrap()
    })
}

#[update]
fn credit_vault(amount: i64) { }

#[update]
fn create_vault(vault_type: VpnRole) {
    VPN_VAULTS.with_borrow_mut(|vaults| {
        let mut cloned = vaults.clone();

        match vaults.iter().find(|v| v.vault_owner == ic_cdk::caller()) {
            Some(_v) => {
                *vaults = cloned;
            },
            _ => {
                cloned.push(VpnVault {
                    vault_type: vault_type,
                    vault_dedicated_ip_servers: vec![],
                    vault_rented_ip_servers: vec![],
                    vault_owner: ic_cdk::caller(),
                    vault_credits: 10
                });
        
                *vaults = cloned;
            }
        }
    })
}

#[update]
fn rent_ip_server() {
    VPN_VAULTS.with_borrow_mut(|vaults| {
        let cloned = vaults.clone();
        *vaults = cloned.iter().map(|v| {
            let mut new_vault_rented_ip_servers = v.vault_rented_ip_servers.clone();

            if v.vault_owner == ic_cdk::caller() {
                assert!(v.vault_credits > 0, "You need to top up credits in order to rent a server");

                let vault = vaults.iter().find(|v| {
                    v.vault_dedicated_ip_servers.len() > 0
                }).unwrap().clone();
                new_vault_rented_ip_servers.push(
                    vault.vault_dedicated_ip_servers.get(0).unwrap().to_string()
                );

                return VpnVault {
                    vault_type: v.vault_type.clone(),
                    vault_dedicated_ip_servers: v.vault_dedicated_ip_servers.clone(),
                    vault_rented_ip_servers: v.vault_rented_ip_servers.clone(),
                    vault_owner: v.vault_owner,
                    vault_credits: v.vault_credits
                }.clone()
            }

            v.clone()
        }).collect();
    })
}

#[update]
fn dedicate_ip_server(ip: String, username: String, pass: String, salt: String) {
    let _encrypted_server = utils::encrypt_server(ip, username, pass, salt);

    VPN_VAULTS.with_borrow_mut(|vaults| {
        let cloned = vaults.clone();
        *vaults = cloned.iter().map(|v| {
            let mut new_vault_dedicated_ip_servers = v.vault_dedicated_ip_servers.clone();
            new_vault_dedicated_ip_servers.push("".to_string());

            if v.vault_owner == ic_cdk::caller() {
                return VpnVault {
                    vault_type: v.vault_type.clone(),
                    vault_dedicated_ip_servers: new_vault_dedicated_ip_servers,
                    vault_rented_ip_servers: v.vault_rented_ip_servers.clone(),
                    vault_owner: v.vault_owner,
                    vault_credits: v.vault_credits
                }.clone()
            }

            v.clone()
        }).collect();
    })
}

// todo
// 1. start_vpn_usage()
// 2. stop_vpn_usage()