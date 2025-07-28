# 🧰 Getting Started

Welcome to **Chronicler** — your digital scriptorium for worldbuilding, notes, and knowledge management.

Chronicler stores your notes as plain Markdown (.md) files in a folder on your computer (called a **vault**).

---

## 📁 Vaults and Files

Your **vault** is just a folder on your computer. You decide where the world is stored, e.g `C:\Users\Michael\MyWorld`.

- ✅ Chronicler will index and render any Markdown file within the vault
- 🔁 Changes in your file system (rename, move, delete) are instantly detected
- 📂 You can create folders, drag-and-drop files, and organize content however you like

---

## ➕ Creating Pages and Folders

- Click the **+ New Page** button in the bottom left sidebar, or right-click on any folder in the file explorer to create a new **page** or **folder**.
- New pages start with a default **YAML frontmatter** block (see below)

---

## 📝 Writing in Markdown

Chronicler uses **Markdown** to format your pages.

- Use `# heading`, `## subheading`, `**bold**`, `*italic*`, `-` for bullet lists, and so on
- Use `[[Page Name]]` wikilinks to link to another page in your vault (if the page doesn't exist, Chronicler will create a placeholder)

---

## 🔖 YAML Frontmatter

Each page can start with an optional **YAML frontmatter** block to store metadata. This is used to generate the wiki-like "infobox" that acts as the page summary card. It's also used to add **tags** to your page (see below).

```yaml
---
title: Rivertown
tags: [city, trade, river]
image: rivertown.jpg
key: value
---
```

| Field   | Description |
|---------|-------------|
| `title` | Display title (otherwise filename is used) |
| `tags`  | List of tags for categorization |
| `image` | Image shown in the infobox |

All frontmatter is optional, and you can also add any custom fields you want (e.g `height`, `age`, `capital`, `population` etc. ) — Chronicler won’t enforce a strict schema.

---

## 🖼️ Infoboxes and Images

You can display an image in a page’s **infobox** by adding the `image` field to the frontmatter:

```yaml
image: rivertown.jpg
```

There are three supported ways to specify the image path:

- ✅ **Relative to the `images/` folder** (recommended)
  - `image: rivertown.jpg` → loads `vault/images/rivertown.jpg`
  - `image: maps/northlands.png` → loads
    `vault/images/maps/northlands.png`
  - This method assumes you have created an `images` folder directly inside the vault root.

- 🗂️ **Relative to the vault root using `../`**
  - `image: ../factions/champions/banner.jpg` → loads
    `vault/factions/champions/banner.jpg`
  - Use this if you want to store images next to your Markdown files instead of inside `images/`

- ⚠️ **Absolute paths** (not recommended)
  - `image: C:/Users/Michael/Pictures/map.png`
  - This only works on your current machine and may break if you move
    the vault

Supported formats: `.jpg`, `.jpeg`, `.png`, `.webp`, `.gif`, `.svg`

---

## 🔗 Linking Between Pages

Use `[[Page Name]]` to link to other pages in your vault.

- Autocompletion helps you insert links quickly
- You can alias links with `[[Silverflow River|Silverflow]]`
- Backlinks are shown in the right sidebar so you can see what links *to* the current page
- When you rename a page, **all links to it are automatically updated**

---

## 🏷️ Tags and Hierarchies

You can tag pages using the `tags:` field in YAML frontmatter:

```yaml
tags: [city, coastal]
```

- Click on any tag to see a list of all pages with that tag

---

## 📥 Importing Word Docs

You can import `.docx` files from Microsoft Word directly into your vault.

- Go to **Settings → Import from .docx** and choose your files
- Formatting (headings, bold, italics, links) is preserved
- Requires Pandoc (Chronicler can download it for you automatically)

---

## 💡 Tips

- Use folders to group related pages (e.g. `places/`, `people/`, `factions/`)
- Pages and folders are ordered alphabetically. If you want to enforce ordering, you can number them (e.g `1_people/`, `2_places/`)
- Use **tags** *and* folders — you can access pages in multiple ways

---

## ❓ Need Help?

- [Join the Discord community!](https://discord.gg/cXJwcbe2b7)
- [GitHub Issues](https://github.com/mak-kirkland/chronicler/issues) for bugs or feature requests

---

Happy chronicling! ✍️ - Michael
