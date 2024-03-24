use crate::{
    actions::{
        add_repo::add_repo_action, add_repo_preset::add_repo_preset_action, apply::apply_hosts,
        apply_android::apply_android_action, backup::backup, del_repo::del_repo_action,
        del_repo_preset::del_repo_preset_action, list_devices::list_devices_action,
        list_repos::list_repos_action, restore::restore_backup,
        restore_android::restore_android_action,
    },
    arguments::Commands,
    AppState,
};

use crate::actions::sync::sync_repositories;

pub async fn exec_command(app_state: &AppState) {
    let args = &app_state.args;

    match args.to_owned().command {
        Commands::Sync(_) => sync_repositories(&app_state).await,
        Commands::Apply => apply_hosts(&app_state),
        Commands::ApplyAndroid(a) => apply_android_action(&app_state, a.device),
        Commands::Backup => backup(&app_state),
        Commands::Restore => restore_backup(&app_state),
        Commands::RestoreAndroid(a) => restore_android_action(&app_state, a.device),
        Commands::AddRepo(a) => add_repo_action(a.repo, &app_state),
        Commands::AddRepoPreset(a) => add_repo_preset_action(a.repo, &app_state),
        Commands::DelRepo(a) => del_repo_action(a.repo),
        Commands::DelRepoPreset(a) => del_repo_preset_action(a.repo),
        Commands::ListRepos => list_repos_action(),
        Commands::ListDevices => list_devices_action(&app_state),
    };
}
