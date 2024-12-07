# Changelog

All notable changes to this project will be documented in this file.
---
## [unreleased]

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
