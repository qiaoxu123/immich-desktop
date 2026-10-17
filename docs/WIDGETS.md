# Desktop widgets / mini app

## MVP

The first iteration offers a compact always-on-top **Immich Mini** window launched from the gallery. This is an app-managed widget, not an OS-native macOS desktop widget or Windows Widgets panel extension.

Planned widgets:

1. Photo of the day / random photo, configurable rotation.
2. On-this-day memories (depends on server support).
3. Album slideshow with album selection.
4. Backup/upload status and quick action.

## Architecture

Widgets use the existing Immich session; do not persist credentials in window settings or URLs. Event hand-off is ephemeral. The production build should keep credentials only in the native process and expose narrow command permissions. Before public release, implement native secure credential storage, better widget session hand-off, persistent window positions, and explicit cache limits.

The first prototype should not run destructive actions or automatic backup. For native desktop integration: macOS WidgetKit is a separate native extension, Windows Widgets require platform-specific integration; keep these as later, independently maintained adapters.
