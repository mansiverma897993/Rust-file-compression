# Rust File Compression Tool

A simple command-line tool written in **Rust** to compress files using **Gzip**.

---

## 🧰 Requirements

- Rust & Cargo installed  
  👉 https://www.rust-lang.org/tools/install

Check installation:

```bash
rustc --version
cargo --version
📥 Get the Project
Copy code
git clone https://github.com/mansiverma897993/Rust-file-compression.git
cd rust-stuff
▶️ Run the Program
Copy code
cargo run -- <input_file> <output_file.gz>
Example (file outside the folder):
Copy code
cargo run -- ../VoteXweb3.pdf file-compression.gz
Example (file in the same folder):
Copy code
cargo run -- VoteXweb3.pdf compressed.gz
✅ Successful Output
text
Compression completed successfully!
The compressed file (.gz) will be created in the current directory.
