# roja

Roja is a TUI music player written in Rust. It's still a WIP.

![Screenshot of roja in action](/assets/demo.png?raw=true "Roja Demo Screenshot")

### TODO

- [x] Add directory of mp3 files, scrape and serialize all metadata.
- [x] List and browse music indexed in the metadata file.
- [x] Play, pause, seek, and view buffered state.
- [x] Play music on a remote server via SSH. This uses [libmpv-rs][libmpv_rs] under the hood.
- [x] Customize the colors by editing config.json file.
- [ ] Fuzzy search through music using the [FST](fst_url) crate.
- [ ] Better documentation >_<

[libmpv_rs]: https://github.com/ParadoxSpiral/libmpv-rs
[fst_url]: https://github.com/BurntSushi/fst
