# Changelog

All notable changes to this project will be documented in this file.
---
## [unreleased]

### Bug Fixes

- update `lib_game_detector` to not fail on a single invalid steam library (#54) - ([1c0dcd4](https://github.com/rolv-apneseth/rofi-games/commit/1c0dcd4781fd19a3eb19e4a0fae9a0f4f52673a6))

---
## [1.16.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.15.0..v1.16.0) - 2025-11-15

### Features

- update `lib_game_detector` for `Itch` support (#51) - ([9c14cea](https://github.com/rolv-apneseth/rofi-games/commit/9c14cea049c52631a6eb4f54fdc2b08e7a40d29a))
- allow specifying environment variables in the config to modify launching commands per entry (#52) - ([f10c10f](https://github.com/rolv-apneseth/rofi-games/commit/f10c10f9d4e08bbbc5434d2806d8e9e83df8cbd8))

---
## [1.15.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.14.0..v1.15.0) - 2025-10-09

### Features

- display the source for each entry next to it's title (configurable) (#49) - ([f606f0d](https://github.com/rolv-apneseth/rofi-games/commit/f606f0d9cbbd149648cd7c47c5cbde76b758f662))

---
## [1.14.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.13.0..v1.14.0) - 2025-10-02

### Features

- sorting order (#47) - ([ad9c88e](https://github.com/rolv-apneseth/rofi-games/commit/ad9c88e5526719d28fe01e0a20ae4c131e454863))

### Miscellaneous Tasks

- **(deps)** bump tracing-subscriber from 0.3.19 to 0.3.20 (#46) - ([087158b](https://github.com/rolv-apneseth/rofi-games/commit/087158b196e14df5e318b930ec3c1d64901cfcc9))

---
## [1.13.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.12.3..v1.13.0) - 2025-09-09

### Documentation

- **(readme)** add home manager installation instructions - ([c382690](https://github.com/rolv-apneseth/rofi-games/commit/c38269065e13a64bf37fd64beb03814acc95e9bf))
- **(readme)** fix alt text on license badge - ([9cfa6a1](https://github.com/rolv-apneseth/rofi-games/commit/9cfa6a1ea0f5bad4fe0e8615cba0cd85246b8b3a))

### Features

- fall back to icons if box art is not available (enabled by default) (#45) - ([a9fcf4e](https://github.com/rolv-apneseth/rofi-games/commit/a9fcf4e4828bcffd869eefc1fb1c110d007e46f4))

---
## [1.12.3](https://github.com/rolv-apneseth/rofi-games/compare/v1.12.2..v1.12.3) - 2025-08-09

### Bug Fixes

- remove `git-cliff` junk from repo and fix action - ([b9fdcf9](https://github.com/rolv-apneseth/rofi-games/commit/b9fdcf969935da5ce51d48bdeecc4a5bb7e43864))
- update `lib_game_detector` to parse Steam's new box art filename - ([0e1f41b](https://github.com/rolv-apneseth/rofi-games/commit/0e1f41b5cdba54585a8ebb40497c531e5a7f6700))

### Continuous Integration

- update `checkout` and `git-cliff` versions - ([2238d4e](https://github.com/rolv-apneseth/rofi-games/commit/2238d4e9dd260199b20e34360fa72757747ff180))

### Documentation

- **(readme)** update license badge - ([5c8d09c](https://github.com/rolv-apneseth/rofi-games/commit/5c8d09cfabe06d99146f5aafac0fb8e012eca9ec))

### Miscellaneous Tasks

- **(deps)** update `rofi-mode` - ([72886f9](https://github.com/rolv-apneseth/rofi-games/commit/72886f95fe9ca18ad61348150e3f02fccd10fc0a))
- bug report issue template - ([4207e24](https://github.com/rolv-apneseth/rofi-games/commit/4207e24792f17a1435512377187da76310dc32a4))

---
## [1.12.2](https://github.com/rolv-apneseth/rofi-games/compare/v1.12.1..v1.12.2) - 2025-04-05

### Bug Fixes

- update `lib_game_detector` to parse all steam shortcuts from all users - ([05bcebd](https://github.com/rolv-apneseth/rofi-games/commit/05bcebddca7258f87b68646c1e7414faa40d7224))

---
## [1.12.1](https://github.com/rolv-apneseth/rofi-games/compare/v1.12.0..v1.12.1) - 2025-03-29

### Bug Fixes

- update `lib_game_detector` to include fix for systems with capitalised "steamapps" directory names - ([b869049](https://github.com/rolv-apneseth/rofi-games/commit/b86904950a571b3ec6f4af257fcca6046aa36851))

---
## [1.12.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.11.0..v1.12.0) - 2025-03-29

### Bug Fixes

- update `lib_game_detector` for Heroic Launcher Epic games title fix - ([f26a3e8](https://github.com/rolv-apneseth/rofi-games/commit/f26a3e8e0247435f3991b104c1ec2fcd5ba582e9))

### Features

- remove box art requirement for entries - ([ecaed85](https://github.com/rolv-apneseth/rofi-games/commit/ecaed85a99de827fe641d496b7508c7f627d1c26))
- add config option to allow hiding entries without defined box art - ([b11fdc3](https://github.com/rolv-apneseth/rofi-games/commit/b11fdc3b651740f4e7f5d9db02701ac341ad7c07))
- add config option to hide an entry - ([0ffcf6d](https://github.com/rolv-apneseth/rofi-games/commit/0ffcf6d76721464d50b232f7bba5924e0159f3b3))

---
## [1.11.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.9..v1.11.0) - 2025-03-24

### Bug Fixes

- disable colours when piping logs to an output file - ([ef389db](https://github.com/rolv-apneseth/rofi-games/commit/ef389dbfec2c8bd70ae49e908b4640eb54dfcec9))

### Features

- update `lib_game_detector` for Heroic Games Launcher sideloaded app support - ([fe03530](https://github.com/rolv-apneseth/rofi-games/commit/fe0353077da78359cb1d82b9bb9518c74baafe3a))

### Miscellaneous Tasks

- update deps - ([0878f82](https://github.com/rolv-apneseth/rofi-games/commit/0878f820493b34ea547ddc8f9db43112b87db565))

---
## [1.10.9](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.8..v1.10.9) - 2025-01-29

### Bug Fixes

- update `lib_game_detector` version to fix additional cases for new Steam `librarycache` structure - ([5ea4a9e](https://github.com/rolv-apneseth/rofi-games/commit/5ea4a9e20837179a2581e9a738c7b8757a0495c7))

---
## [1.10.8](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.7..v1.10.8) - 2025-01-24

### Bug Fixes

- update `lib_game_detector` version to fix detection of Steam games with the new `librarycache` directory structure - ([95f34f9](https://github.com/rolv-apneseth/rofi-games/commit/95f34f94f02bbc83c582ba09eb9d07992e1555c4))

### Documentation

- **(readme)** add tip about escaping spaces in custom launch commands - ([b80cc1f](https://github.com/rolv-apneseth/rofi-games/commit/b80cc1ffe521f371d9500e8407244ce776dc879b))

### Miscellaneous Tasks

- update deps - ([c876636](https://github.com/rolv-apneseth/rofi-games/commit/c876636db7a37744583fcde56194f2de428b1bec))

---
## [1.10.7](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.6..v1.10.7) - 2024-12-09

### Bug Fixes

- **(readme)** update warning about custom entries and games with the same title to match code - ([9186858](https://github.com/rolv-apneseth/rofi-games/commit/91868589f21d362dcf6a8e9963792e3d78a3c180))
- fully custom entry detection broken by previous update - ([9c8dbca](https://github.com/rolv-apneseth/rofi-games/commit/9c8dbca4e028e17ef1a3ab4355c186613609cb04))

### Documentation

- **(readme)** update warning about matching titles - ([a37e85d](https://github.com/rolv-apneseth/rofi-games/commit/a37e85d4a21d4ffbf22954ea47a974ed4796034f))

---
## [1.10.6](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.5..v1.10.6) - 2024-12-07

### Bug Fixes

- linting - ([26bb0eb](https://github.com/rolv-apneseth/rofi-games/commit/26bb0ebf2bf011322da24d1f2966220acb9525e0))

### Continuous Integration

- run the newly included tests - ([175e507](https://github.com/rolv-apneseth/rofi-games/commit/175e507eb404c84284c143f684383685b16b68bf))

### Features

- **(just)** add format and lint commands - ([c2a54dc](https://github.com/rolv-apneseth/rofi-games/commit/c2a54dc69108b36579cedd3d07e1b5f600fea50c))

### Refactoring

- changes for v0.0.12 of `lib_game_detector` - ([634b0dd](https://github.com/rolv-apneseth/rofi-games/commit/634b0dde6049e0ffad1dbd7d3be3978cc62c5b85))

### Tests

- basic testing for custom entry adding function - ([2cd2126](https://github.com/rolv-apneseth/rofi-games/commit/2cd2126876d030a58338677a18ad51c80342eb2d))

---
## [1.10.5](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.4..v1.10.5) - 2024-11-08

### Bug Fixes

- update `lib_game_detector` to fix an issue with Lutris game detection - ([55703c8](https://github.com/rolv-apneseth/rofi-games/commit/55703c8ec85f74f4f1a16f50425f880f764921ad))

---
## [1.10.4](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.3..v1.10.4) - 2024-11-07

### Bug Fixes

- update `lib_game_detector` to fix issues with Lutris fallback paths - ([ef138d2](https://github.com/rolv-apneseth/rofi-games/commit/ef138d28b465aaef0d5dffa84a6bc16ab65b60b3))

---
## [1.10.3](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.2..v1.10.3) - 2024-11-06

### Bug Fixes

- update `lib_game_detector` to patch issues with Lutris path detection and an error while looking for non-Steam games - ([238c88d](https://github.com/rolv-apneseth/rofi-games/commit/238c88dffc98eb9770d4eb672da6872a235498a4))

---
## [1.10.2](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.1..v1.10.2) - 2024-08-10

### Documentation

- **(readme)** update manual install description with info about the `rofi` plugins directory - ([ba529a7](https://github.com/rolv-apneseth/rofi-games/commit/ba529a71671d90058c796d7cf3d12bc91cb2fd27))

### Refactoring

- change fallback plugins directory if the `pkg-config` command doesn't return anything - ([e8e03b4](https://github.com/rolv-apneseth/rofi-games/commit/e8e03b4213e6ef2cb14f78cebe81696d667adee9))

---
## [1.10.1](https://github.com/rolv-apneseth/rofi-games/compare/v1.10.0..v1.10.1) - 2024-08-04

### Documentation

- **(readme)** update Steam shortcuts section - no need to restart - ([c2b9121](https://github.com/rolv-apneseth/rofi-games/commit/c2b91215fb1326ed359a2f249bcc11caa8532e86))

### Refactoring

- for custom entries, make `path_game_dir` optional - ([a36f253](https://github.com/rolv-apneseth/rofi-games/commit/a36f253a65e604c2be8c54f558645a82a98f491a))

---
## [1.10.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.9.2..v1.10.0) - 2024-07-21

### Features

- update `lib_game_detector` for Steam shortcuts (non-Steam games) support - ([41cd92f](https://github.com/rolv-apneseth/rofi-games/commit/41cd92f29bd5435f763082d65afcad302a9a7059))

---
## [1.9.2](https://github.com/rolv-apneseth/rofi-games/compare/v1.9.1..v1.9.2) - 2024-06-24

### Bug Fixes

- only apply default display name if none is given - ([028240b](https://github.com/rolv-apneseth/rofi-games/commit/028240bb05ce00abc70c53eb707c396f2dc6572e))

### Continuous Integration

- include `git-cliff` action for creating and maintaining a changelog - ([e4b548d](https://github.com/rolv-apneseth/rofi-games/commit/e4b548d3014433098a2e1c699a09bb7799c9561b))

### Style

- update default theme to display the `prompt` widget, but hide the widget for the smaller theme - ([d9de281](https://github.com/rolv-apneseth/rofi-games/commit/d9de281add29b670712f431a990af1683b91bdf0))

---
## [1.9.1](https://github.com/rolv-apneseth/rofi-games/compare/v1.9.0..v1.9.1) - 2024-05-23

### Miscellaneous Tasks

- update dependencies (including `lib_game_detector`) - ([95654f7](https://github.com/rolv-apneseth/rofi-games/commit/95654f77ccafa57e13be52e5cf6d90b0884f4f95))

### Refactoring

- format `justfile` and provide default option for theme for testing - ([bd7407c](https://github.com/rolv-apneseth/rofi-games/commit/bd7407c1f26864fb2283b1328d78401c62a1f9c6))
- update types to match new `lib_game_detector` version - ([9683b77](https://github.com/rolv-apneseth/rofi-games/commit/9683b7755a92ffa07834d3e7ff94445c99bf1f05))

---
## [1.9.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.8.0..v1.9.0) - 2024-05-07

### Documentation

- **(readme)** fix html code block - ([a012836](https://github.com/rolv-apneseth/rofi-games/commit/a0128364b6b2b33c806453ff49d2bb5fa4ffdfd1))

### Features

- add support for Prism Launcher and ATLauncher (through updating lib_game_detector) - ([6b67742](https://github.com/rolv-apneseth/rofi-games/commit/6b677429a03d7c69c1d11be9e8f2c03c7f8bae38))

---
## [1.8.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.7.0..v1.8.0) - 2024-04-24

### Documentation

- **(readme)** add configuration section - ([0e5f009](https://github.com/rolv-apneseth/rofi-games/commit/0e5f009ae76487101577ccc302c7a0865f913832))

### Features

- add configuration system for defining custom entries - ([bb4e421](https://github.com/rolv-apneseth/rofi-games/commit/bb4e42193173e7dbc12b0917538cd9ece51e0929))

### Style

- fix default theme so it remains 2 rows of games, and add rounded corners to smaller theme - ([5dadad4](https://github.com/rolv-apneseth/rofi-games/commit/5dadad48004e405dda30403ba60c577398a82324))

---
## [1.7.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.8..v1.7.0) - 2024-04-14

### Features

- update lib_game_detector, including support for flatpaks and fixing a couple issues with parsing Lutris games - ([58aed58](https://github.com/rolv-apneseth/rofi-games/commit/58aed58a4fb7cad31362b2f48ed3f1a6318ca623))

---
## [1.6.8](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.7..v1.6.8) - 2024-04-04

### Bug Fixes

- remove `Cargo.lock` from `.gitignore` - ([feb8c74](https://github.com/rolv-apneseth/rofi-games/commit/feb8c74c19ed7589110bf12b5bdb1f363c55ba50))
- bump version in `Cargo.lock` - ([3b96fcb](https://github.com/rolv-apneseth/rofi-games/commit/3b96fcb6cabc2e9b03e4ecbf25e0df439570c2c0))
- `Cargo.lock`, as it was previously built using the local `lib_game_detector` repo - ([e4dca52](https://github.com/rolv-apneseth/rofi-games/commit/e4dca5222133c7c8956c42cc58b6dd27d5e8656c))

---
## [1.6.7](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.6..v1.6.7) - 2024-02-27

### Bug Fixes

- remove duplicate element-icon definition from theme - ([2cd8f2a](https://github.com/rolv-apneseth/rofi-games/commit/2cd8f2a9be5580304a47fe31f8a1d1f5ffa5a54e))

### Miscellaneous Tasks

- update dependencies and bump version - ([edb88fb](https://github.com/rolv-apneseth/rofi-games/commit/edb88fb24a4b3582bd8ed8bb98a82fb51b9fa2cb))

---
## [1.6.6](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.5..v1.6.6) - 2024-02-25

### Bug Fixes

- rofi version check in justfile - ([3c8049b](https://github.com/rolv-apneseth/rofi-games/commit/3c8049bdb2f19d629ada63e7988a127d61387424))

### Documentation

- **(README)** update install and keybinds sections - ([b2aca8e](https://github.com/rolv-apneseth/rofi-games/commit/b2aca8e752138b79fed69448f82c4b79091b6c79))

### Features

- add github workflow for testing - ([c339d3e](https://github.com/rolv-apneseth/rofi-games/commit/c339d3ee7165bbb99993f846e5359168346339c9))

---
## [1.6.5](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.4..v1.6.5) - 2024-02-03

### Improvements

- reduce final library file size - ([75151b2](https://github.com/rolv-apneseth/rofi-games/commit/75151b2f63039b46d8baccbcbc4612cbc27688f8))

---
## [1.6.4](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.3..v1.6.4) - 2024-01-04

### Bug Fixes

- issues with plugin path as `PLUGINS_DIR` will not consistently have a `/` at the end - ([6c7e6af](https://github.com/rolv-apneseth/rofi-games/commit/6c7e6af059f488ce7e0042b0bff9fd10eae3a6cd))

### Documentation

- **(readme)** update to use `just` - ([0e9044e](https://github.com/rolv-apneseth/rofi-games/commit/0e9044ea7b6858188893d8c030c36225551f279f))
- remove unused CHANGELOG - ([6930163](https://github.com/rolv-apneseth/rofi-games/commit/6930163e99f157c49671d3120f6c970b56fe9a96))

### Features

- add some basic utility commands to `justfile` - ([239303a](https://github.com/rolv-apneseth/rofi-games/commit/239303a6f9db186cf9049412834f58b2c5167022))
- some additional utility commands for the `justfile` - ([8ef889d](https://github.com/rolv-apneseth/rofi-games/commit/8ef889d6b4c1764b403fef15bbea30eb32291add))

### Refactoring

- switch from `Makefile` to `justfile` - ([9bd3bfa](https://github.com/rolv-apneseth/rofi-games/commit/9bd3bfa93a127520fa9e2de45ef85be508379b3a))

---
## [1.6.3](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.2..v1.6.3) - 2023-12-31

### Bug Fixes

- don't include theme readme in install - ([22e6e64](https://github.com/rolv-apneseth/rofi-games/commit/22e6e64285aff4544a32f2ac1ca782b433266a4e))

---
## [1.6.2](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.1..v1.6.2) - 2023-12-31

### Documentation

- **(readme)** update demo image - ([a7c18eb](https://github.com/rolv-apneseth/rofi-games/commit/a7c18ebea63ef951dc100887c0fe95ae0841e33b))
- **(themes)** add screenshots - ([fb7fdbe](https://github.com/rolv-apneseth/rofi-games/commit/fb7fdbe36e6d021c5ccea34de0fcea263cb9c9ed))

### Features

- provide default themes - ([d9f1aef](https://github.com/rolv-apneseth/rofi-games/commit/d9f1aefffc0b7ace954aa626401f720aabe7b9fd))

---
## [1.6.1](https://github.com/rolv-apneseth/rofi-games/compare/v1.6.0..v1.6.1) - 2023-11-30

### Refactoring

- separate out build step in Makefile - ([9d008ea](https://github.com/rolv-apneseth/rofi-games/commit/9d008ea965c3289e72918aba3b93759dcf979da0))

---
## [1.6.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.5.0..v1.6.0) - 2023-11-30

### Features

- support `rofi` versions with changes newer than the base `1.7.5` - ([3556e2f](https://github.com/rolv-apneseth/rofi-games/commit/3556e2fe292cf57cb811029a48a4177b8076b734))

---
## [1.5.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.4.1..v1.5.0) - 2023-11-16

### Documentation

- **(readme)** include Bottles in sources - ([138add8](https://github.com/rolv-apneseth/rofi-games/commit/138add8047f8c03c906e6c0daa7fd9f5a748a630))

### Features

- Bottles support, from updating to lib_game_detector v0.0.3 - ([5efdd26](https://github.com/rolv-apneseth/rofi-games/commit/5efdd26996bff26cac772b58634c5d084918d365))

---
## [1.4.1](https://github.com/rolv-apneseth/rofi-games/compare/v1.4.0..v1.4.1) - 2023-11-05

### Refactoring

- move from env_logger to tracing - ([6844400](https://github.com/rolv-apneseth/rofi-games/commit/6844400e87be225b765bf8dd52c65e1220ba52b6))

---
## [1.3.1](https://github.com/rolv-apneseth/rofi-games/compare/v1.3.0..v1.3.1) - 2023-10-31

### Miscellaneous Tasks

- update lib_game_detector version - ([a13c88b](https://github.com/rolv-apneseth/rofi-games/commit/a13c88bb17667ee973c96aa57e2a0e2e98a91062))

---
## [1.3.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.2.1..v1.3.0) - 2023-09-09

### Features

- extract game parsing functionality into a separate library (also added Heroic Amazon support) - ([e73470b](https://github.com/rolv-apneseth/rofi-games/commit/e73470bd2189730bd1efffda20dfc1c39554ae85))

---
## [1.2.1](https://github.com/rolv-apneseth/rofi-games/compare/v1.1.3..v1.2.1) - 2023-07-02

### Bug Fixes

- move parse_value_from_json_line function to helpers - ([7cd9d03](https://github.com/rolv-apneseth/rofi-games/commit/7cd9d034ecd773c668b793df61845ec1bc9fb7ac))
- skip duplicate entries in game-paths.json and fix parsing of entries by ignoring commas - ([adb5438](https://github.com/rolv-apneseth/rofi-games/commit/adb5438b025e4d7e12f6c39a93adc5665838076a))

### Features

- add support for Lutris games - ([ff1d4a7](https://github.com/rolv-apneseth/rofi-games/commit/ff1d4a778d666b9bc03f2848ee4658c6828de190))

### Improvements

- pass only necessary paths to game launcher structs and let them handle specific paths in their constructors - ([b8ef9c3](https://github.com/rolv-apneseth/rofi-games/commit/b8ef9c3acad47cc3c611ce18ba1f1202887b178d))

---
## [1.1.3](https://github.com/rolv-apneseth/rofi-games/compare/v1.1.1..v1.1.3) - 2023-06-19

### Bug Fixes

- stop license file being placed in project root - ([0cde28b](https://github.com/rolv-apneseth/rofi-games/commit/0cde28b7e626e35654d6aba75f9b5a55150a16d3))
- skip extra title fields defined for Legendary games which were causing out-of-bounds crashes - ([0db1f71](https://github.com/rolv-apneseth/rofi-games/commit/0db1f718a424dea8f3b6a33a6daed79167f19a25))

### Features

- add basic logging system - ([6e0de6b](https://github.com/rolv-apneseth/rofi-games/commit/6e0de6bda902567cbd981cf29e8d7e8c78097299))

---
## [1.1.1](https://github.com/rolv-apneseth/rofi-games/compare/v1.1.0..v1.1.1) - 2023-05-10

### Bug Fixes

- don't crash if there is an error reading games from a particular launcher - ([8f14e5a](https://github.com/rolv-apneseth/rofi-games/commit/8f14e5ae41ffa76e572bf63de1f05b10e607a665))

---
## [1.1.0](https://github.com/rolv-apneseth/rofi-games/compare/v1.0.2..v1.1.0) - 2023-05-10

### Features

- support heroic games launcher - ([2f0773a](https://github.com/rolv-apneseth/rofi-games/commit/2f0773aa66f632cc8b19cc3ffaf330c114212515))

---
## [0.0.2] - 2023-05-01

### Bug Fixes

- ensure presence of desktop entries folder before certain actions - ([1d0b70d](https://github.com/rolv-apneseth/rofi-games/commit/1d0b70d9e68f8345229f63ef40a656fef621e739))
- include extra metadata in Cargo.toml - ([71a50e3](https://github.com/rolv-apneseth/rofi-games/commit/71a50e38e64f2210564c32a8a6aacc2972c98403))

### Features

- add clap for argument parsing - ([e754814](https://github.com/rolv-apneseth/rofi-games/commit/e75481432146d1f1a4b4feaec9d2f1fca218091e))

<!-- generated by git-cliff -->
