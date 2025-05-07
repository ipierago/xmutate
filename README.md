# xmutate

**xmutate** is a fast, flexible command-line tool for mutating structured data. It reads JSON input, applies one of several built-in mutators, and writes the result to an output file.

---

## ✨ Features

- 🔧 Pluggable mutators (e.g. `summarize`, `filter`)
- 📦 Simple CLI interface powered by [`clap`](https://docs.rs/clap)
- 🧪 Strong error handling with [`thiserror`](https://docs.rs/thiserror)
- 💡 Easily extensible to support custom data operations

---

## 🚀 Usage

```bash
xmutate summarize --input data.json --output summary.json --group-by category

xmutate filter --input data.json --output result.json --key status --op eq --value active
