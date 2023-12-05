use crate::{
    actions::{
        add_repo::add_repo_action, add_repo_preset::add_repo_preset_action, apply::apply_hosts,
        apply_android::apply_android_action, backup::backup, del_repo::del_repo_action,
        del_repo_preset::del_repo_preset_action, list_devices::list_devices_action,
        list_repos::list_repos_action, restore::restore_backup,
        restore_android::restore_android_action,
    },
    arguments::{Args, Commands},
};

use crate::actions::sync::sync_repositories;

pub fn exec_command(args: &Args) {
    match args.to_owned().command {
        Commands::Sync(_) => sync_repositories(args.to_owned()),
        Commands::Apply => apply_hosts(args.to_owned()),
        Commands::ApplyAndroid(a) => apply_android_action(a.device),
        Commands::Backup => backup(),
        Commands::Restore => restore_backup(),
        Commands::RestoreAndroid(a) => restore_android_action(a.device),
        Commands::AddRepo(a) => add_repo_action(a.repo, args.to_owned()),
        Commands::AddRepoPreset(a) => add_repo_preset_action(a.repo, args.to_owned()),
        Commands::DelRepo(a) => del_repo_action(a.repo),
        Commands::DelRepoPreset(a) => del_repo_preset_action(a.repo),
        Commands::ListRepos => list_repos_action(),
        Commands::ListDevices => list_devices_action(),
    };
}
