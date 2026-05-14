# Assets

Static visual assets used by the root README and other documentation.

## Files

| File | Purpose |
|------|---------|
| `p.svg` | Animated ASCII-art letter `P` rendered by the root `README.md`. |

## Editing

Keep README-facing assets self-contained when possible. `p.svg` includes its own styles and animation so it can be embedded with:

```html
<img src="assets/p.svg" alt="Animated ASCII art letter P" width="640">
```

Run this after changing SVG markup:

```bash
xmllint --noout assets/p.svg
```
