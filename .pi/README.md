# bevyout Pi Harness

This is a deliberately small, model-agnostic Pi bootstrap. It selects no
provider or model.

## Launch

From any directory:

```powershell
& C:\Users\V\Projects\Rust\bevyout\Tools\pi-bevyout.ps1
```

All Pi arguments are preserved:

```powershell
.\Tools\pi-bevyout.ps1 --provider openai --model <model-id>
```

The launcher changes to the repository root, disables automatic root context
loading, approves project-local Pi settings for that run, and explicitly loads
`.pi/PI.md`. It does not write user-global settings. Detailed project knowledge
remains on demand in `AGENTS.md`, repository skills, local Bevy documentation,
and the project wiki.

## Validate

No model request or running viewer is required:

```powershell
.\Tools\pi-bevyout.ps1 --offline --version
git diff --check
```
