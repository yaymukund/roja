## Roadmap to v1

- [ ] **Fetch metadata and index files**
  - [x] Configure paths
  - [x] ~~SSH library integration~~ Just pass ssh:// url to mpv
  - [ ] Loading bar

- [ ] **Playlist**
  - [x] Build data models for songs
  - [ ] PlaylistView
  - [ ] User interactions

- [ ] **Folder list**
  - [x] Build data model for folders
  - [ ] FoldersView
  - [x] User interactions

- [x] **Player**
  - [x] Play/pause indicator
  - [x] Current track title, as pulled from metadata
  - [x] Progress bar
  - [x] Idle state

- [ ] **Search**
  - [x] Decide on a fast search library
  - [ ] Debounce/throttle user input
  - [ ] Wire up to the playlist
  - [ ] SearchView

- [ ] **Theming**
  - [x] User configurable theme
  - [ ] \*Bonus\* Visualizations
  - [ ] \*Bonus\* Album art
