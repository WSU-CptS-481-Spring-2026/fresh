# File Explorer

Fresh includes a built-in file explorer.

## Configuration

The file explorer width can be configured in `file_explorer.width` (fraction of viewport, e.g. `0.3` = 30%). The value is clamped to 10%–50% at runtime, matching the range enforced when resizing with the mouse. This ensures the sidebar stays usable whether set via config or drag.

*   **Toggle Sidebar:** Use `Ctrl+B` to show/hide the file explorer sidebar.
*   **Focus:** Use `Ctrl+E` to switch focus between the file explorer and editor.
*   **Navigation:** Use the arrow keys to move up and down the file tree.
*   **Open Files:** Press `Enter` to open the selected file and focus the editor. Single-click opens a file but keeps focus on the explorer; double-click opens and focuses the editor.
*   **Gitignore Support:** The file explorer respects your `.gitignore` file, hiding ignored files by default.
*   **Visibility Toggles:** Use "Toggle Hidden Files" and "Toggle Gitignored Files" from the command palette. These settings persist to config across sessions.

## Configuration

- **`file_explorer.width`** — Fraction of the terminal width used by the sidebar (for example, `0.3` means 30%). The value is **clamped at runtime** to **10%–50%** so it stays consistent with mouse-drag resize of the sidebar. See [Configuration](../configuration/index.md) for other file explorer options.
