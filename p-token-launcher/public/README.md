# Launcher Public Assets

Browser files served by `p-token-launcher/server.mjs`.

## Files

| File | Purpose |
|------|---------|
| `index.html` | Workbench UI structure for launch plans, quotes, perp intents, mint inspection, and registry output. |
| `app.js` | Browser-side form handling, API calls, tab switching, and JSON rendering. |
| `styles.css` | Responsive dark UI styling for the local workbench. |

The frontend is intentionally unsigned. It only sends planner requests to local API routes and renders the returned JSON.
