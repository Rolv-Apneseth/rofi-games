use log::{debug, error, warn};
use std::{
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use super::{Game, Launcher};

#[derive(Debug)]
pub struct GamePathsJsonData {
    executable_name: String,
    run_id: String,
}

#[derive(Debug)]
pub struct GameYmlData {
    executable_name: String,
    name: String,
    slug: String,
}

#[derive(Debug)]
pub struct CombinedGameData {
    run_id: String,
    name: String,
    slug: String,
}

pub struct Lutris {
    path_lutris_games_dir: PathBuf,
    path_lutris_box_art_dir: PathBuf,
    path_lutris_game_paths_json: PathBuf,
}

impl Lutris {
    pub fn new(path_config_home: &Path, path_cache_home: &Path) -> Self {
        let path_lutris_games_dir = path_config_home.join("lutris/games");
        let path_lutris_box_art_dir = path_cache_home.join("lutris/coverart");
        let path_lutris_game_paths_json = path_cache_home.join("lutris/game-paths.json");

        debug!(
            "Lutris games path exists: {}",
            path_lutris_games_dir.is_dir()
        );
        debug!(
            "Lutris box art path exists: {}",
            path_lutris_box_art_dir.is_dir()
        );
        debug!(
            "Lutris game-paths.json file exists: {}",
            path_lutris_game_paths_json.is_file()
        );

        Lutris {
            path_lutris_games_dir,
            path_lutris_box_art_dir,
            path_lutris_game_paths_json,
        }
    }

    /// Parse data from the game-paths.json file for Lutris
    fn parse_game_paths_json(&self) -> Result<Vec<GamePathsJsonData>, ()> {
        let mut data = Vec::new();

        let game_paths_json_file = File::open(&self.path_lutris_game_paths_json).map_err(|e| {
            error!(
                "Error with reading game-paths.json file at {:?}:\n{e}",
                self.path_lutris_game_paths_json
            )
        })?;

        for line in BufReader::new(game_paths_json_file)
            .lines()
            .flatten()
            .skip(1)
        {
            if line.contains('}') {
                break;
            }

            let parsed_data = line
                .split_once(": ")
                .map(|t| {
                    [t.0, t.1]
                        .iter()
                        .map(|s| s.trim().replace(['"', ','], ""))
                        .collect::<Vec<String>>()
                })
                .ok_or_else(|| error!("Error parsing data from game-paths.json file"))?;

            // Skip duplicate entries, which aren't valid json but still happen for some reason
            if data
                .iter()
                .any(|d: &GamePathsJsonData| d.run_id == parsed_data[0])
            {
                continue;
            }

            let parsed_executable_name = parsed_data[1]
                .rsplit_once('/')
                .map(|f| f.1)
                .ok_or_else(|| error!("Error extracting executable name"))?;

            data.push(GamePathsJsonData {
                run_id: parsed_data[0].to_owned(),
                executable_name: parsed_executable_name.to_owned(),
            });
        }

        Ok(data)
    }

    /// Parse data from the Lutris games directory, which contains 1 .yml file for each game
    fn parse_games_dir(&self) -> Result<Vec<GameYmlData>, ()> {
        let mut data = Vec::new();

        for path in read_dir(&self.path_lutris_games_dir)
            .map_err(|e| {
                warn!(
                    "Error with reading
games directory for Lutris: {e:?}"
                )
            })?
            .flatten()
        {
            if let Some(parsed_data) = self.parse_game_yml(path.path())? {
                data.push(parsed_data);
            }
        }

        Ok(data)
    }

    /// Parse data from a given game .yml file path
    fn parse_game_yml(&self, path_game_yml: PathBuf) -> Result<Option<GameYmlData>, ()> {
        let mut name = String::new();
        let mut slug = String::new();
        let mut executable_name = String::new();

        let game_yml_file = File::open(&path_game_yml).map_err(|e| {
            error!(
                "Error with reading game-paths.json file at {:?}:\n{e}",
                self.path_lutris_game_paths_json
            )
        })?;

        for line in BufReader::new(game_yml_file).lines().flatten().skip(1) {
            if !name.is_empty() && !slug.is_empty() && !executable_name.is_empty() {
                break;
            };

            if line.contains("exe: ") {
                executable_name = line
                    .rsplit_once('/')
                    .map(|t| t.1)
                    .ok_or_else(|| debug!("Error parsing exe line in game yml file. Line: {line}"))?
                    .to_owned();
            } else if line.contains("game_slug: ") {
                slug = line
                    .split_whitespace()
                    .last()
                    .ok_or_else(|| debug!("Error parsing slug in game yml file. Line: {line}",))?
                    .to_owned();
            } else if line.contains("name: ") {
                name = line
                    .split_once(": ")
                    .map(|t| t.1)
                    .ok_or_else(|| {
                        debug!("Error parsing name line in game yml file. Line: {line}")
                    })?
                    .to_owned();
            }
        }

        if name.is_empty() || slug.is_empty() || executable_name.is_empty() {
            debug!(
                "Could not find relevant data fields for Lutris game from file:
{path_game_yml:?}"
            );

            Ok(None)
        } else {
            Ok(Some(GameYmlData {
                name,
                slug,
                executable_name,
            }))
        }
    }

    /// Get all relevant game data by combining data from the game-paths.json file and each games
    /// .yml file.
    /// Matching of the data from these sources is done using the executable path of the game,
    /// which is the only thing defined in both sources
    pub fn get_game_data(&self) -> Result<Vec<CombinedGameData>, ()> {
        let mut combined_data = Vec::new();

        let game_paths_data = self.parse_game_paths_json()?;
        let game_yml_data = self.parse_games_dir()?;

        for path_data in game_paths_data {
            if let Some(combined_datum) = game_yml_data
                .iter()
                .find(|g| g.executable_name == path_data.executable_name)
                .map(|yml_data| CombinedGameData {
                    run_id: path_data.run_id,
                    name: yml_data.name.clone(),
                    slug: yml_data.slug.clone(),
                })
            {
                combined_data.push(combined_datum);
            }
        }

        println!("{combined_data:?}");

        Ok(combined_data)
    }
}

impl Launcher for Lutris {
    /// Get all available Lutris games
    fn get_games(&self) -> Result<Vec<Game>, ()> {
        let mut games = Vec::new();

        let game_data = self.get_game_data()?;
        for game_datum in game_data {
            let cover_path = self
                .path_lutris_box_art_dir
                .join(format!("{}.jpg", game_datum.slug));

            if !cover_path.is_file() {
                warn!(
                    "Skipping {} as {cover_path:?} does not exist",
                    game_datum.slug
                );
                continue;
            }

            let launch_command = format!(
                "env LUTRIS_SKIP_INIT=1 lutris lutris:rungameid/{}",
                game_datum.run_id
            );

            games.push(Game::new(game_datum.name, launch_command, cover_path))
        }

        if games.is_empty() {
            warn!("No games (at least not with sufficient data) found for Lutris launcher");
        }

        Ok(games)
    }
}
