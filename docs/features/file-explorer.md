# File Explorer

Fresh includes a built-in file explorer.

## Configuration

The file explorer width can be configured in `file_explorer.width` (fraction of viewport, e.g. `0.3` = 30%). Valid values are **10%–50%** (0.1–0.5). When you change this in the **Settings** UI and save, values outside that range are **rejected** and an error message is shown in the status bar instead of applying. Values loaded from config files at startup are still **clamped** to that range for safety so the editor always starts in a usable state.

*   **Toggle Sidebar:** Use `Ctrl+B` to show/hide the file explorer sidebar.
*   **Focus:** Use `Ctrl+E` to switch focus between the file explorer and editor.
*   **Navigation:** Use the arrow keys to move up and down the file tree.
*   **Open Files:** Press `Enter` to open the selected file and focus the editor. Single-click opens a file but keeps focus on the explorer; double-click opens and focuses the editor.
*   **Gitignore Support:** The file explorer respects your `.gitignore` file, hiding ignored files by default.
*   **Visibility Toggles:** Use "Toggle Hidden Files" and "Toggle Gitignored Files" from the command palette. These settings persist to config across sessions.
