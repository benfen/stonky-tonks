use db::balance::ModBalanceHistory;
use db::establish_connection;
use db::user::User;

pub fn calculate_balances() -> Result<(), String> {
    let connection = establish_connection();

    User::retrieve_all_users(&connection)
        .iter()
        .for_each(|user| {
            ModBalanceHistory::create_history(&user, &connection);
        });

    Ok(())
}
