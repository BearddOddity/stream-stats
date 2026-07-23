# Stream Stats

Live viewer count, follower total, and stream stats across Twitch and Kick

Standalone Tauri v2 + React app — also vendored into
[StreamerSuite](https://github.com/BearddOddity/StreamerSuite) as one of its
launcher tools (see StreamerSuite's `VENDORING.md`).

## Branches

- `main` — the standalone app. Build/run independently with `npm install && npm run tauri dev`.
- `streamersuite-integration` — the StreamerSuite-adapted version (shared theme/settings, and shared backend where applicable). Not independently buildable; it's the staging copy for what's vendored into StreamerSuite's `src/apps/stream-stats/`.

## Development

```sh
npm install
npm run tauri dev
```

`src/design-system/` is vendored from StreamerSuite's own `src/design-system/`
(the shared UI kit used by StreamerSuite's launcher-native tools). Keep it in
sync manually when StreamerSuite's design system changes.
