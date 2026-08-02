---
name: react-shadcn-ui-development
description: Use this skill when building UI features in React with shadcn/ui, Tailwind CSS, and TypeScript for this workspace.
---

# React + shadcn/ui development

## When to use

- Adding or updating React components in src/components
- Implementing UI using shadcn/ui primitives and Tailwind utility classes
- Creating reusable feature components or small composable UI modules
- Updating the desktop app layout, panels, or interaction states

## Repository-specific guidance

- Follow the existing component organization under src/components/ui and src/components/features.
- Prefer small, composable components over large nested structures.
- Reuse shared utilities from src/lib/utils when possible.
- Keep the UI consistent with the existing shadcn/ui style and Tailwind CSS 4 patterns.
- Do not introduce new UI frameworks unless explicitly requested.

## Recommended workflow

1. Review the existing component structure before adding a new UI element.
2. Prefer composing existing primitives rather than building custom UI from scratch.
3. Keep state and rendering logic simple and localized.
4. Ensure the component is reusable and easy to test visually.
5. Verify the build with bun run build.

## Verification

- Run bun run build after frontend changes.
- Check that styling and TypeScript compile cleanly.
- If a change affects the desktop experience, validate it in the app.
