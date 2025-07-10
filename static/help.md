# 📝 Writing Pages in Markdown

Chronicler saves your worldbuilding notes as simple, human-readable **Markdown files**.

You can optionally add **YAML frontmatter** to include metadata like titles, tags, and images.

Chronicler also supports `[[wikilinks]]` for easy linking between pages — just like in a wiki.

---

## 📄 File Format

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

## 🧠 Frontmatter Fields

All frontmatter is optional — use it if it's helpful for you! Chronicler won't enforce any structure, so you're free to customize as much as you like.

However, three fields have special behavior:

| Field   | Description |
|---------|-------------|
| `title` | The display title for the page. If omitted, the filename is used. |
| `tags`  | A list of tags to categorize the page. |
| `image` | The filename of an image (e.g., `rivertown.jpg`) used in the infobox. |

### Notes:
- You can define whatever fields you like, and as many as you like.
- Tags can also be anything you choose.
- Images must be placed in an `images/` folder inside your vault root.
  For example, if your vault is at `C:\Users\Frank\World`, your image should go in `C:\Users\Frank\World\images\`.
- Use `[[Page Name]]` to link to another page. If the page doesn’t exist yet, Chronicler will create a placeholder for it.
