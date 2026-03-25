#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    Address, Env, Map, Vec, Symbol
};

#[contract]
pub struct UniSplitContract;

#[contracttype]
#[derive(Clone)]
pub struct Bill {
    pub total: i128,
    pub share: i128,
    pub members: Vec<Address>,
}

#[contracttype]
pub enum DataKey {
    Bill,
    Paid(Address),
}

#[contractimpl]
impl UniSplitContract {

    // Create group bill
    pub fn create_bill(env: Env, total: i128, members: Vec<Address>) {

        let count = members.len() as i128;
        let share = total / count;

        let bill = Bill {
            total,
            share,
            members,
        };

        env.storage().instance().set(&DataKey::Bill, &bill);
    }

    // Pay bill
    pub fn pay_bill(env: Env, user: Address, amount: i128) {

        user.require_auth();

        let bill: Bill = env
            .storage()
            .instance()
            .get(&DataKey::Bill)
            .unwrap();

        if amount < bill.share {
            panic!("Not enough payment");
        }

        env.storage()
            .instance()
            .set(&DataKey::Paid(user), &true);
    }

    // Check payment status
    pub fn is_paid(env: Env, user: Address) -> bool {

        env.storage()
            .instance()
            .get(&DataKey::Paid(user))
            .unwrap_or(false)
    }
}
stellar contract invoke \
--id CA3CWG2YBH6BQ33DZAPG5APGGBLIF63K2C4L6GW5VQY5A5VTRYBLOUVB \
--source student \
--network testnet \
-- \
create_bill \
--total 300 \
--members '["GBIA7KQ4XO6KJQSRZYRKV7732UDX4WLDF3TLRY6WSVWM4VPK3LKOETOE"]'