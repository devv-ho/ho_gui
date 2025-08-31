# Git Workflow and Commit Rules

## Commit Message Format

### Standard Format:
```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### Types:
- **feat**: New feature implementation
- **fix**: Bug fixes
- **docs**: Documentation changes
- **style**: Code style changes (formatting, no logic change)
- **refactor**: Code refactoring (no new features or bug fixes)
- **test**: Adding or updating tests
- **chore**: Build process, dependency updates, tooling

### Scopes (optional):
- **math**: Math types and operations
- **widgets**: Widget implementations
- **rendering**: Rendering pipeline
- **app**: Application framework
- **layout**: Layout system
- **input**: Input handling
- **style**: Styling system

### Examples:
```
feat(math): implement Point struct with new() and zero() methods

fix(math): correct const fn syntax for Point methods

test(math): add comprehensive unit tests for Point struct

docs: update .gitignore for Rust library project

refactor(lib): comment out incomplete modules during development
```

## Commit Rules

### When to Commit:
1. **After completing a task** (e.g., Point struct fully implemented and tested)
2. **After fixing compilation errors** that affect multiple files
3. **After adding significant documentation** 
4. **Before starting major refactoring**

### What to Include in Each Commit:
- **Single logical change** - one feature, one fix, one refactor
- **All related files** - implementation + tests + documentation
- **Working state** - code should compile (use `cargo check`)
- **Tested changes** - run `cargo test` before committing

### Commit Frequency:
- **Task completion**: After finishing each task from TASK_X_X_X.md
- **Milestone completion**: After major implementations (e.g., all math types)
- **Before switching contexts**: When moving between different modules
- **Daily**: At minimum, daily progress commits

### Branch Strategy:
- **main**: Stable, working code only
- **feature/task-x-x-x**: Individual task branches (optional for small features)
- **dev**: Integration branch for ongoing development (if needed)

## Pre-Commit Checklist:
- [ ] `cargo check` passes
- [ ] `cargo test` passes (if tests exist)
- [ ] `cargo fmt` applied
- [ ] `cargo clippy` warnings addressed
- [ ] Commit message follows format
- [ ] All files staged appropriately

## Current Development Workflow:
1. **Receive task assignment** in `docs/tasks/`
2. **Implement feature** according to specification
3. **Write tests** and ensure they pass
4. **Update documentation** if needed
5. **Run quality checks** (check, test, fmt, clippy)
6. **Commit with proper message**
7. **Push to remote** when task complete
8. **Request code review** from senior engineer

## Emergency Commits:
For work-in-progress or broken builds, use:
```
wip: <description of current work>

[explanation of what's broken or incomplete]
```

But **avoid pushing WIP commits** to main branch.