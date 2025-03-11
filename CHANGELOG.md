## Version 0.2.1 (2025-03-20)

### Changed

- Write debug logs to the filesystem in addition to stdout.
- Prevent console window from appearing on Windows operating systems.

## Version 0.2.0 (2025-03-04)

Previously, WorkTimer was written with [AutoHotkey](https://www.autohotkey.com) and was based on [an app by Neil Cicierega](http://neilblr.com/post/58757345346). The original code, though posted publicly, was provided without a license.

This version is **_completely rewritten from scratch with Rust_**, allowing the project to be published with an open source license.

### Added

- You can now save your elapsed time, window position, and settings to a project file. This allows you to, for example, track your video projects independently from each other.

## Version 0.1.1 (2017-06-08)

### Added

- New feature where user can save the program's current position. Now you can return the timer to that perfect spot in the corner.

### Fixed

- An algorithm mistake with displaying programs in the context menu where a set amount of characters were removed rather than left. Context menu should be smaller as a result now.

## Version 0.1.0 (2017-05-30)

Original application can be found here: http://neilblr.com/post/58757345346.

### Improvements

- More compact UI -- borderless and draggable
- All windows/classes of the same program are now tracked together
- Up to nine programs can be saved and followed
- Miscellaneous algorithm and logic improvements
