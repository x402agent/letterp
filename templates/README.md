# Templates

Reusable starter folders for Pinocchio programs and p-token planning tools. These are source templates, not audited production programs.

## Template Map

| Template | Use when | Promoted folder |
|----------|----------|-----------------|
| `escrow` | Starting a make/take/refund token escrow program. | None yet |
| `vault` | Starting a minimal deposit/withdraw vault program. | None yet |
| `p-agent-token` | Starting an agent-bound p-token program with launch-curve hooks. | `../p-agent-token` |
| `p-token-launcher` | Starting an unsigned p-token launch planning UI/API. | `../p-token-launcher` |

## How to Use

1. Copy the template directory to a new project folder.
2. Replace placeholders such as `{{project_name}}` and `{{crate_name}}`.
3. Keep the local Pinocchio path dependencies or rewrite them to the copied project's vendored Pinocchio location.
4. Fill in PDA seed checks, account owner validation, CPI transfers, tests, and deployment-specific program IDs.
5. Run the relevant local checks before adding the copied folder to a workspace.

The promoted folders in the repo show how `p-agent-token` and `p-token-launcher` are adapted into concrete project directories.
