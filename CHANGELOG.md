# Changelog

## [1.2.0](https://github.com/izyuumi/xcode-discord-rpc/compare/v1.1.0...v1.2.0) (2025-09-18)


### Features

* **config:** add support for global config file and env vars ([579c43a](https://github.com/izyuumi/xcode-discord-rpc/commit/579c43a20e99ddf4b736efdc48a62f2d237f0051)), closes [#28](https://github.com/izyuumi/xcode-discord-rpc/issues/28)
* **lang:** add support for Objective-C files ([4767ce4](https://github.com/izyuumi/xcode-discord-rpc/commit/4767ce47a0b5c3297b1e29429360ed44413cf159)), closes [#27](https://github.com/izyuumi/xcode-discord-rpc/issues/27)

## [1.1.0](https://github.com/izyuumi/xcode-discord-rpc/compare/v1.0.0...v1.1.0) (2025-02-28)


### Features

* embed default configuration as compile-time constant ([cea3684](https://github.com/izyuumi/xcode-discord-rpc/commit/cea3684a1b600b16a57f70b195574bf20a46c604))

## [1.0.0](https://github.com/izyuumi/xcode-discord-rpc/compare/v0.3.5...v1.0.0) (2025-02-25)


### ⚠ BREAKING CHANGES

* `show-file` and `show-project` arguments has been renamed to `hide-file` and `hide-project`

### Features

* add configurable Xcode update interval and improve Discord connection handling ([5070fec](https://github.com/izyuumi/xcode-discord-rpc/commit/5070fec77cd1c6389222fa30a61f8e6a898cdc6e))
* add configuration management ([f142ed4](https://github.com/izyuumi/xcode-discord-rpc/commit/f142ed450f67947bf9bea72d6f73244378bc096d))
* add custom error variant and improve main function error handling ([040b3e5](https://github.com/izyuumi/xcode-discord-rpc/commit/040b3e52a653ed51fc6094f5266677c4ec6a44a0))
* enhance logging levels based on debug configuration ([be81828](https://github.com/izyuumi/xcode-discord-rpc/commit/be8182882a4ddad148c808b6935c608a3c38ae9a))
* improve file language handling ([407acec](https://github.com/izyuumi/xcode-discord-rpc/commit/407acecac76ad4aa0e6cd8d2a2cc0b2783783a43))
* integrate SimpleLogger for improved logging ([474bf11](https://github.com/izyuumi/xcode-discord-rpc/commit/474bf118008a49433169a6660ee300aeeb3d28a4))
* introduce custom error handling with Result type and Error enum ([7476f2a](https://github.com/izyuumi/xcode-discord-rpc/commit/7476f2aa3a66debf7ccb0b1c3540b98b794db480))
* rename wait_time with update_interval ([a09f20b](https://github.com/izyuumi/xcode-discord-rpc/commit/a09f20bd2261f04c38834aa2f493650cab1a3f39))


### Bug Fixes

* enhance error handling in run_osascript function ([6b8db8a](https://github.com/izyuumi/xcode-discord-rpc/commit/6b8db8a55c5983ee2f93cf966f084d4e7521f14b))
* improve error handling for Discord IPC client initialization ([b1b5634](https://github.com/izyuumi/xcode-discord-rpc/commit/b1b5634050dcf4dbb1e8f3a0bd6d379b1e5eaf86))
* rename wait_time to update_interval in default configuration ([783020a](https://github.com/izyuumi/xcode-discord-rpc/commit/783020a5f7acbd3efb5c69c3c8eda49f9b33d1bf))
* streamline output handling in AppleScript execution functions ([21c8aee](https://github.com/izyuumi/xcode-discord-rpc/commit/21c8aee91afea0080485313e62b4b51b0b5aa04a))


### Performance Improvements

* skip discord connection when xcode is not running ([f44d54a](https://github.com/izyuumi/xcode-discord-rpc/commit/f44d54a20c8f5dceb09e6f6c3c9493824e110962))

## [0.3.5](https://github.com/izyuumi/xcode-discord-rpc/compare/v0.3.4...v0.3.5) (2025-02-17)


### Bug Fixes

* add tag name to homebrew workflow for release versioning ([0379766](https://github.com/izyuumi/xcode-discord-rpc/commit/0379766f9309db8dd12d8fdd4453b193e2e98a54))

## [0.3.4](https://github.com/izyuumi/xcode-discord-rpc/compare/v0.3.3...v0.3.4) (2025-02-16)


### Bug Fixes

* update homebrew tap and download URL in brew.yml workflow ([de5c6d1](https://github.com/izyuumi/xcode-discord-rpc/commit/de5c6d198e55b002d6c8d6e1b2b76378f76dc6d5))

## [0.3.3](https://github.com/izyuumi/xcode-discord-rpc/compare/v0.3.2...v0.3.3) (2025-02-16)


### Bug Fixes

* activity timestamps needs to be in milliseconds ([ccb6a52](https://github.com/izyuumi/xcode-discord-rpc/commit/ccb6a52aa2676b8ab11ee19b13e28e5ed1d9cc6a)), closes [#10](https://github.com/izyuumi/xcode-discord-rpc/issues/10)
* no need to change release body ([206e94b](https://github.com/izyuumi/xcode-discord-rpc/commit/206e94b1134ad50e1303e6ccd96b5184bcce85d6))

## [0.3.2](https://github.com/izyuumi/xcode-discord-rpc/compare/v0.3.1...v0.3.2) (2025-02-13)


### Bug Fixes

* remove redundant create release step in brew.yml workflow ([e70a345](https://github.com/izyuumi/xcode-discord-rpc/commit/e70a345a83cc4f24c7a05b561c537c9a9ef8f863))

## [0.3.1](https://github.com/izyuumi/xcode-discord-rpc/compare/v0.3.0...v0.3.1) (2025-02-13)


### Bug Fixes

* update version retrieval in brew.yml workflow ([8f0aaf2](https://github.com/izyuumi/xcode-discord-rpc/commit/8f0aaf269bfb02203f1499a30e5b7e51db296bb6))

## [0.3.0](https://github.com/izyuumi/xcode-discord-rpc/compare/v0.2.1...v0.3.0) (2025-02-13)


### Features

* show idle status ([1dbc5b7](https://github.com/izyuumi/xcode-discord-rpc/commit/1dbc5b7bb00a6b4bdc32c8b71659f4d53af9382a)), closes [#9](https://github.com/izyuumi/xcode-discord-rpc/issues/9)


### Bug Fixes

* improve log messages for activity updates ([c2422e2](https://github.com/izyuumi/xcode-discord-rpc/commit/c2422e2c83cd429d5b09b320932e871ef57093e7))
* update log message ([52b6989](https://github.com/izyuumi/xcode-discord-rpc/commit/52b698916c0e5f7aa5a293e18e349ffc5d7ef305))
