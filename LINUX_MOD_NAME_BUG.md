# Linux Bug: Mod Names Showing Full Path

## Issue
On Linux, mod names display the entire file path instead of just the filename.

## Root Cause
**TSX Issue** in `repak-x/src/App.tsx`

The frontend extracts the mod filename using Windows-specific path separator:

```tsx
// Line 2041
const modName = mod.mod_name || mod.custom_name || mod.path.split('\\').pop() || ''

// Line 2049  
const displayName = (mod.custom_name || mod.path.split('\\').pop() || '').toLowerCase()
```

On Linux, paths use forward slashes (`/`), so `split('\\')` returns the entire path as a single element, and `.pop()` returns the full path instead of just the filename.

## Fix Required
Replace `split('\\')` with a cross-platform solution:

```tsx
// Option 1: Split on both separators
mod.path.split(/[/\\]/).pop()

// Option 2: Use a helper function
const getFilename = (path: string) => path.split(/[/\\]/).pop() || path
```

## Affected Lines
- `App.tsx:2041` - LODs_Disabler filter
- `App.tsx:2049` - Search query filter

## Status
Documented for TSX fix (backend Rust code is correct - it sends full PathBuf which serializes correctly on both platforms).
