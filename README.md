# File Deduplication Tool

A command-line tool for finding and managing duplicate files in a directory using various hash algorithms.

## Features

- Scan directories recursively for files
- Compute file hashes using multiple algorithms (SHA256, Blake3, XXHash)
- Generate JSON reports of duplicate files
- Optionally quarantine duplicate files (keeping one original)
- Parallel processing for fast performance
- Progress tracking during file processing

## Installation

1. Ensure you have Rust installed (version 1.70 or higher recommended)
2. Clone this repository
3. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

```sh
./file-deduplicator [OPTIONS]
```

### Options

| Option | Description | Default |
|--------|-------------|---------|
| `-d, --directory` | Directory to scan for duplicates | Current directory (.) |
| `-o, --output` | Output report file name | duplicates.json |
| `-a, --algorithm` | Hash algorithm to use (sha256, blake3, xxhash) | blake3 |
| `-q, --quarantine` | Quarantine duplicates (keeps one original) | false |
| `--quarantine-dir` | Quarantine directory path | quarantine |

### Examples

1. Basic scan with default settings:
   ```sh
   ./file-deduplicator
   ```

2. Scan specific directory with SHA256:
   ```sh
   ./file-deduplicator -d /path/to/files -a sha256
   ```

3. Scan and quarantine duplicates:
   ```sh
   ./file-deduplicator -q --quarantine-dir /path/to/quarantine
   ```

## Hash Algorithms

- **SHA256**: Cryptographic hash, slow but secure (64-character hex output)
- **Blake3**: Modern cryptographic hash, very fast (64-character hex output)
- **XXHash**: Non-cryptographic hash, extremely fast (16-character hex output)

## Output

The tool generates a JSON report file containing groups of duplicate files, with each group sharing the same hash value. Example:

```json
[
  {
    "hash": "abc123...",
    "files": ["/path/to/file1.txt", "/path/to/file2.txt"]
  }
]
```

## Quarantine Mode

When enabled with `-q/--quarantine`, the tool will:
1. Keep the first file in each duplicate group
2. Move all other duplicates to the quarantine directory
3. Rename files if naming conflicts occur in quarantine

## Testing

Run the test suite with:
```sh
cargo test
```

