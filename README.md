Wrapper for yt-dlp to easily download and organize music

## Installation and Run

### 1. Install globally
Run this command in the project folder to install the app to your system:

```bash
cargo install --path .
```

### 2. Run the app

You can now run it from any terminal directory:

```bash
rmdl
```

### 3. Batch Download from a File

```bash
rmdl links.txt
```
The utility first searches for the file in the current directory; if it doesn't find the file there, it searches the Desktop directory

### 4. Browser Cookies & Authorization (Important Note)

To bypass YouTube's anti-bot protection (403 Forbidden errors), download playlists, and access age-restricted content, rmdl uses cookies from a local browser.

How to use it correctly:
1. Log into a YouTube account in the configured browser (it's highly recommended to use a secondary/dummy account so it doesn't pause your music on your phone).
2. CRITICAL: The browser MUST be fully closed before running rmdl. If the browser is open, the cookie database will be locked, and yt-dlp will throw an error.

##

Note: If you switch browsers (e.g., from chromium to a specific firefox profile), remember to change the --cookies-from-browser argument inside cmd_builder.rs and run cargo install --path . again to apply the changes.