# 📈 Sizr

```text
      _            
 ___ (_) ____ _ __ 
/ __|| ||_  /| '__|
\__ \| | / / | |   
|___/|_|/___||_|  
```

📈 Sizr: A fast, Rust-based tool to analyze file system usage with colorful output, file aggregation, exclusions, and directory comparisons.

## Demo

![Example](resources/screenshot.png)


## ✨ Features

### Features

- **Colored Output**: Displays results with colors based on the `LS_COLORS` environment variable for clear and organized visuals.
- **File System Tree View**: Visualizes the file system structure, making it easy to understand folder and file hierarchy.
- **Small File Aggregation**: Option to group smaller files together for a more concise view of storage usage.
- **Exclusions**: Ability to exclude specific files or directories from analysis, enhancing customization.
- **Directory Comparison**: Compare multiple directories to quickly identify storage differences.
- **High Performance**: Built in Rust for speed and efficiency across file system operations.

## 🚀 Installation

To install **sizr**, simply clone the repository and follow the instructions below:

```bash
git clone git@github.com:trinhminhtriet/durs.git
cd sizr

cargo build --release
cp ./target/release/sizr /usr/local/bin/
```

Running the below command will globally install the `sizr` binary.

```bash
cargo install sizr
```

## 💡 Usage

```
 $ sizr --help
Usage: sizr [options] <path> [<path>..]

Options:
    -d, --depth [DEPTH] show directories up to depth N (def 1)
    -a, --aggr [N[KMG]] aggregate smaller than N B/KiB/MiB/GiB (def 1M)
    -s, --summary       equivalent to -da, or -d1 -a1M
    -u, --usage         report real disk usage instead of file size
    -b, --bytes         print sizes in bytes
    -f, --files-only    skip directories for a fast local overview
    -x, --exclude NAME  exclude matching files or directories
    -H, --no-hidden     exclude hidden files
    -A, --ascii         ASCII characters only, no colors
    -h, --help          show help
    -v, --version       print version number
```

## 🗑️ Uninstallation

Running the below command will globally uninstall the `sizr` binary.

```bash
cargo uninstall sizr
```

Remove the project repo

```bash
rm -rf /path/to/git/clone/sizr
```

## 🤝 How to contribute

We welcome contributions!

- Fork this repository;
- Create a branch with your feature: `git checkout -b my-feature`;
- Commit your changes: `git commit -m "feat: my new feature"`;
- Push to your branch: `git push origin my-feature`.

Once your pull request has been merged, you can delete your branch.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
