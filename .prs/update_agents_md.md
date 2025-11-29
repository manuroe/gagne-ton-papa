# PR Description Template

<!-- 
AGENTS: Before sending a PR, populate this file with the proposed title and body.
Notify the user to review and edit this file.
Once the user approves, use the content of this file for the `gh pr create` command.
-->

## Title
Update agents.md with improved guidelines

## Body
This PR updates `agents.md` to be more structured and compatible with various AI agents (Gemini, Claude, ChatGPT, etc.).

**Changes:**
- **Explicit System Instruction**: Added a header explicitly instructing agents to treat the file as the source of truth.
- **Structured Sections**: Organized content into "Project Overview", "Core Rules & Behavior", "Tech Stack & Standards", and "Workflows".
- **Clear Directives**: Used strong language ("MUST", "CRITICAL") to ensure agents follow the rules, especially regarding the PR workflow and self-improvement.
- **Project Structure**: Added `src/` (TUI app) to the architecture overview.
- **Quota Management**: Refined the rule to skip resuming requests after a quota limit.

## Prompts Used
<!-- 
AGENTS: List the prompts used to generate this code, as per agents.md.
-->
- "The last PR did not follow @agents.md#L17-18. How can make you and other AI agents respect them?
About PR description review, create a temporary file with the PR description to be sent that I can edit. You will send the saved version.
Add a line so that agents can amend agents.md based on the interactions we add in the prompts to better narrows my desires. The agent will do this when I ask to send a PR"
- "make agents.md compatible with most agents including claude"
- "make a PR with those changes starting from main"
