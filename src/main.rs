#![allow(
    clippy::let_underscore_drop,
    clippy::module_name_repetitions,
    clippy::too_many_lines,
    clippy::uninlined_format_args
)]

mod log;
mod name;

use crate::log::Log;
use crate::name::CrateName;
use anyhow::Result;
use db_dump::crate_owners::OwnerId;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::btree_map::{BTreeMap as Map, Entry};
use std::collections::BTreeSet as Set;
use std::io::Write;
use std::path::Path;
use std::process;
use termcolor::{ColorChoice, StandardStream};

const DB_DUMP: &str = "./db-dump.tar.gz";
const SQUATTED_CSV: &str = "./squatted.csv";

#[derive(Serialize, Deserialize)]
struct Row {
    #[serde(rename = "crate")]
    name: CrateName,
    user: String,
    version: Option<Version>,
}

fn main() {
    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
    if let Err(err) = try_main(&mut stderr) {
        writeln!(stderr.error(), "{err}");
        process::exit(1);
    }
}

fn try_main(stderr: &mut StandardStream) -> Result<()> {
    let db_dump = Path::new(DB_DUMP);
    if !db_dump.is_file() {
        write!(stderr.error(), "Database dump file does not exist: ");
        write!(stderr.red(), "{}", db_dump.display());
        let _ = writeln!(
            stderr,
            "\nDownload one from https://static.crates.io/db-dump.tar.gz",
        );
        process::exit(1);
    }

    let mut crate_id_to_name = Map::new();
    let mut crate_name_to_id = Map::new();
    let mut versions = Map::new();
    let mut crate_owners = Map::new();
    let mut users = Map::new();
    db_dump::Loader::new()
        .crates(|row| {
            let name = CrateName::new(row.name);
            crate_id_to_name.insert(row.id, name.clone());
            crate_name_to_id.insert(name, row.id);
        })
        .versions(|row| match versions.entry(row.crate_id) {
            Entry::Vacant(entry) => {
                entry.insert(row);
            }
            Entry::Occupied(mut entry) => {
                if row.created_at > entry.get().created_at {
                    entry.insert(row);
                }
            }
        })
        .crate_owners(|row| {
            if let OwnerId::User(user_id) = row.owner_id {
                crate_owners
                    .entry(row.crate_id)
                    .or_insert_with(Vec::new)
                    .push(user_id);
            }
        })
        .users(|row| {
            users.insert(row.id, row.gh_login);
        })
        .load(db_dump)?;

    let mut squatted = Set::new();
    for row in csv::Reader::from_path(SQUATTED_CSV)?.into_deserialize() {
        let row: Row = row?;
        let Some(crate_id) = ({
            crate_name_to_id.get(&row.name)
        }) else {
            // Crate deleted from crates.io
            continue;
        };
        let Some(max_version) = ({
            versions.get(crate_id)
        }) else {
            // All versions deleted from crates.io
            continue;
        };
        if let Some(version) = row.version {
            if version != max_version.num {
                // Most recent published version is newer than the one from the csv
                continue;
            }
        }
        squatted.insert(row.name);
    }

    for (crate_id, version) in &versions {
        if version.num.pre.contains("reserved")
            || version.num.build.contains("reserved")
            || version.num.pre.contains("placeholder")
            || version.num.build.contains("placeholder")
        {
            squatted.insert(crate_id_to_name[crate_id].clone());
        }
    }

    let mut writer = csv::Writer::from_path(SQUATTED_CSV)?;
    let mut leaderboard = Map::new();
    for name in squatted {
        let crate_id = crate_name_to_id[&name];
        let version = &versions[&crate_id];
        let owners = if let Some(published_by) = version.published_by {
            vec![published_by]
        } else {
            crate_owners
                .get(&crate_id)
                .map_or_else(Vec::new, Vec::clone)
        };
        let user = if owners.len() == 1 {
            users[&owners[0]].clone()
        } else {
            String::new()
        };
        writer.serialize(Row {
            name,
            user,
            version: Some(version.num.clone()),
        })?;
        for user_id in owners {
            *leaderboard.entry(user_id).or_insert(0) += 1;
        }
    }

    let mut leaderboard = Vec::from_iter(leaderboard);
    leaderboard.sort_by_key(|(_user, count)| Reverse(*count));
    println!("Leaderboard:");
    for (user_id, count) in leaderboard.iter().take(16) {
        println!("{}, {}", count, users[user_id]);
    }

    Ok(())
}
