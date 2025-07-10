# 🧭 Chronicler

> **Your digital scriptorium – where knowledge links together.** 🧙‍♂️

![Latest Release](https://img.shields.io/github/v/release/mak-kirkland/chronicler?label=release)

**Chronicler** is a free desktop app for note-takers, storytellers, researchers, and creative minds. It saves your notes as plain Markdown files on your computer — no subscriptions, no logins, and no internet required. Your thoughts stay in your hands.

🎉 Download the [Latest Release](https://github.com/mak-kirkland/chronicler/releases/latest)!

❤️ [Support on Patreon](https://patreon.com/ChroniclerNotes) or [Buy Me a Coffee](https://buymeacoffee.com/chronicler) to help fund development.

---

![image](https://github.com/user-attachments/assets/6786c1fd-755b-428c-8bab-65503212c4bf)

---

## ✨ Features (Planned & In Progress)

### ✍️ Writing & Markdown

- Uses simple **Markdown files and folders**
- Clean editor with **auto-save** and **live preview**

### 🔗 Linking & Organization

- **Tags** with **hierarchies**
- Internal **[[wikilinks]]** with autocomplete
- **Backlinks** to trace relationships between ideas
- Smart **auto-indexing** and **link updates** on rename

### 📇 Templates & Infoboxes

- Add structure with optional **infoboxes** (e.g., people, places, topics)
- Define your own reusable **templates**

### 🗂️ Hierarchies & Categorization

- Access the same note through multiple paths (e.g., by tag, topic or filesystem location)
- Smart indexing supports flexible organization

### 🖼️ Media Support

- Embed local images via `![[images/file.jpg]]`
- Drag-and-drop support

### 📥 Importing from Word

Chronicler supports importing `.docx` files directly, making it easy to bring your existing notes into the app.

- Converts Word formatting into clean **Markdown**
- Preserves headings, lists, bold/italic text, and links
- Works great for writers and worldbuilders migrating old content
- Once imported, content is fully editable and linkable like any other page

### 🔐 Private & Offline

- 100% offline — **no cloud**, no vendor lock-in
- Files are just **Markdown + YAML**, portable and future-proof

---

## 🧭 Philosophy

> Your notes. Your files. Your rules.

Chronicler is built on three core principles:

- **Ownership**: Your data is stored in plain text files on your local machine. You are not locked into a proprietary format or cloud service.
- **Privacy**: The app works 100% offline. What you write is for your eyes only.
- **Flexibility**: A simple, powerful set of tools for linking ideas, designed to adapt to your way of thinking.

---

## 🚀 Getting Started

- **Download the latest release**: Head to the [Releases Page](https://github.com/mak-kirkland/chronicler/releases) and download the installer for your operating system (Windows, macOS, Linux).
- **Create a Vault**: A "vault" is the folder on your computer where Chronicler will store all your notes. You can create a new folder or select an existing one.
- **Start Writing!**: Create your first note and start linking your ideas.

---

## 📝 Writing Pages in Markdown

Chronicler stores your worldbuilding content in simple Markdown files with optional YAML frontmatter. This makes it easy to edit, version, and back up your world.

### 📄 File Format

Each page is a Markdown `.md` file. You can optionally begin the file with a YAML frontmatter block like this:

```markdown
---
title: Rivertown
tags: [city, river, trade hub]
image: rivertown.jpg
---

# Rivertown

**Rivertown** is a vibrant settlement along the [[Silverflow River]].

## Economy

The town thrives on river trade and fishing exports from [[Silverflow River|Silverflow]].
```

---

### 🧠 Frontmatter Fields

All frontmatter is optional — use it if it's helpful for you! Chronicler won't enforce any structure, so you're free to customize as much as you like.

However, three fields have special behavior:

| Field   | Description |
|---------|-------------|
| `title` | The display title for the page. If omitted, the filename is used. |
| `tags`  | A list of tags to categorize the page. |
| `image` | The filename of an image (e.g., `rivertown.jpg`) used in the infobox. |

**Notes**:
- You can define whatever fields you like, and as many as you like.
- Tags can also be anything you choose.
- Images must be placed in an `images` folder inside your vault root.
  For example, if your vault is at `C:\Users\Frank\World`, your image should go in `C:\Users\Frank\World\images\`.
- Use `[[Page Name]]` to link to another page. If the page doesn’t exist yet, Chronicler will create a placeholder for it.

---

## ❤️ Support Chronicler's Development

Chronicler is a free, open-source project driven by a passion for privacy and user ownership. It will never have subscriptions or cloud-based features that lock you in.

Your financial support directly funds development time, helping to build new features, fix bugs, and design a better user experience.

- 👉 [Join on Patreon](https://patreon.com/ChroniclerNotes) to vote on features.

- 👉 [Buy Me a Coffee](https://buymeacoffee.com/chronicler) for a one-time donation.

---

## 🛠️ Tech Stack

- **Frontend**: Svelte 5
- **Backend**: Rust
- **Packaging**: Tauri 2.0

---

## 📫 Get in Touch

- Bugs & Feature Requests: Please open an issue on [GitHub Issues](https://github.com/mak-kirkland/chronicler/issues)
- Email: [mak.kirkland@proton.me](mailto:mak.kirkland@proton.me)
- Discord: *(Coming soon!)*
