## [0.5.0] 2023-11-22

- Added support for getting/setting comment - PR: #27
- Added full date capabilities - PR: #34
- Fixed `Id3v2Tag::{*year}` methods - PR: #21 fix #9
- Fixed incorrect access to track fields on disc getters - PR: #26
- Fixed album artist getting copied to artist field - PR: #29
- Fixed incorrect option handling for conversion of `ID3v2Tag` to `AnyTag` - PR: #37 fix #36
- Changed `Tag::read_from_path` return type to `Result<Box<dyn AudioTag + Send + Sync>>` - PR: #21 fix #8
- Removed `unwrap` in `Tag::read_from_path` - PR: #21 fix #7
- Removed needless borrowed reference when getting `Picture` - PR: #28

 * Thanks to @Serial-ATA, @cdown, @microtonez, @aybruh00, and @BSteffaniak

## [0.4.1] 2022-08-02

- Add AudioTagEdit::{set_,remove_}composer - PR: #19 fix #4 

 * Thanks to @Serial-ATA and @ChousX

## [0.4.0] 2022-08-01

- Merged audiotags2 to audiotags - Party! - PR: #18

* Thanks to @martpie

## [0.3.1] 2022-05-25

- Upgraded `id3` from 1.0.3 to 1.1.0

## [0.3.0] 2022-05-25

- Added support for `duration`
- Added support for `genre`
- Upgraded `id3` from 0.5.1 to 1.0.3
- Upgrade `mp4ameta` from 0.6 to 0.11
- Execute tests from tmp directory to avoid repo corruption

## [0.2.7182] 2020-10-29

- Improve docs
- Ergonomic conversions

## [0.2.718] 2020-10-27

- downcasting

## [0.2.71] 2020-10-27

- Remove use of `Cow`

## [0.2.5] 2020-10-27

- Naive implementation of config

## [0.2.3] 2020-10-27

- multiple artists

## [0.2.2] 2020-10-27

- Conversion between tag types without macro; removed the macro introduced in v0.2.0

## [0.2.1] 2020-10-27

- Improved error handling

## [0.2.0] 2020-10-26

- conversion between tag types (naive and unstable implementation)
