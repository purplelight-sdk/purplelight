# Application descriptor

The application descriptor is the file `agera-application.json` at the project's root directory.

## ID

**Syntax**

```json
{
    "id": "com.application.id"
}
```

## Installation files

Installation files can be added to the descriptor using glob patterns, specifying paths to include and exclude.

Each entry in the `include` and `exclude` arrays is allowed to be in either of the following forms:

- A string resolving to a file path.
- A string resolving to a directory path — from which files are recursively copied to the application installation directory.

<!--

- `{ "copy": "path" }` — Copies *path* in the installation directory, where *path* is a glob pattern. Each directory and file is copied with the path as is.
- `{ "copy": "path1", "into": "path2" }` — Copies *path1* into *path2* in the installation directory, where *path1* is a glob pattern and *path2* is not a glob pattern. Examples:
  - if you specify `"copy": "assets/**/*.png"` and `"into": "assets"`, it recursively copies every `*.png` file into the `assets` directory in the installation directory.
- `{ "copy": "path1", "as": "path2" }` — Copies *path1* as *path2*, using **no** glob pattern.

-->

**Syntax**

```json
{
    "installFiles": {
        "include": [],
        "exclude": []
    },
}
```