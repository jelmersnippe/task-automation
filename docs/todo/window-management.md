# macOS Window Management Module

**Goal**: Programmatically move and resize application windows using AppleScript.
No third-party tools required.

## Design

```dsl
window.move("iTerm2", "top_left")
window.move("iTerm2", "top_right")
window.move("iTerm2", "center")
window.move("iTerm2", "left_half")
window.move("iTerm2", "right_half")
window.move("iTerm2", "full_screen")
```

Named positions (`top_left`, `top_right`, `bottom_left`, `bottom_right`, `center`, `left_half`, `right_half`, `full_screen`)
map to coordinate calculations based on screen resolution detected via `system_profiler SPDisplaysDataType`.

## Steps

| Step | Description | Files |
|------|-------------|-------|
| 1 | Create `src/modules/window/mod.rs` | new `src/modules/window/mod.rs` |
| 2 | Implement `move(app, position)` — named positions to AppleScript `set bounds` calls | `src/modules/window/mod.rs` |
| 3 | Screen resolution detection via `system_profiler SPDisplaysDataType` | `src/modules/window/mod.rs` |
| 4 | Register module in `main.rs` | `src/main.rs`, `src/modules/mod.rs` |
