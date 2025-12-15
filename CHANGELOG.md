# Changelog - Code Cleanup and Documentation Updates

## Summary
This update addresses several assumptions from the inventory report, removing template code, improving documentation, and properly configuring platform-specific file locations.

## Changes Made

### 1. Frontend Cleanup
**Files Modified:**
- `src/main.ts` - Removed greet template code
- `index.html` - Simplified HTML to reflect system tray-only nature

**Rationale:** The application UI is the system tray icon. The HTML window is not shown to users, so the Tauri template greet functionality was unnecessary and could cause errors.

**Impact:** Cleaner codebase, no more references to non-existent `greet` Rust command.

---

### 2. Mobile Code Removal
**Files Modified:**
- `src-tauri/src/lib.rs` - Removed mobile entry point

**Rationale:** Mobile is not a target platform for Tea. The mobile entry point was dead code.

**Impact:** Clearer intent that Tea is desktop-only.

---

### 3. State File Location Improvements
**Files Modified:**
- `src-tauri/src/persistence.rs` - Updated `get_state_file_path()`

**Changes:**
- **Windows**: Now uses `%LOCALAPPDATA%\tea\state.json` instead of executable directory
- **macOS**: Now uses `~/Library/Application Support/tea/state.json`
- **Linux**: Unchanged (`~/.config/tea/state.json`)
- **Fallback**: Executable directory for unknown platforms

**Rationale:** Windows applications should store user data in AppData, not the executable directory. This follows platform conventions and works better with user permissions.

**Impact:** 
- Proper Windows behavior
- Better separation of code and data
- Works correctly with non-admin installs
- **Note**: Existing users will need to reconfigure their settings after update (one-time migration)

---

### 4. F15 Key Choice Documentation
**Files Modified:**
- `src-tauri/src/wake_service.rs` - Added documentation explaining F15 choice

**Documentation Added:**
```
## Why F15?
F15 was chosen because it is non-standard on most keyboards and therefore
unlikely to conflict with application shortcuts or user workflows. Most
applications don't bind actions to F15, making it safe to simulate without
interrupting user work.
```

**Rationale:** Users and developers should understand why F15 specifically is used.

**Impact:** Better code documentation for future maintainers.

---

### 5. Autostart Documentation
**Files Modified:**
- `src-tauri/src/main.rs` - Added platform-specific comments for autostart

**Documentation Added:**
- Windows: Registry at `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`
- macOS: LaunchAgent plist in `~/Library/LaunchAgents`
- Linux: Desktop file in `~/.config/autostart`

**Rationale:** Developers and advanced users should understand how autostart works on each platform.

**Impact:** Better understanding of the autostart mechanism.

---

### 6. Test Suite Overhaul
**Files Modified:**
- `tests/e2e.test.ts` - Completely rewritten
- `tests/frontend.test.ts` - Completely rewritten
- `tests/autostart.test.ts` - New file

**Changes:**
- Removed all greet-related test code (589 lines in frontend.test.ts, 543 lines in e2e.test.ts)
- Created focused tests for system tray-only application structure
- Added autostart documentation tests
- All 29 tests passing

**Tests Now Cover:**
- Basic HTML structure validation
- Absence of interactive elements (correct for tray-only app)
- Content validation
- Accessibility basics
- Autostart behavior documentation
- Platform-specific autostart locations

**Rationale:** The old tests were testing non-existent greet functionality. New tests reflect the actual application design.

**Impact:** Test suite now validates what the application actually does.

---

### 7. README Updates
**Files Modified:**
- `README.md`

**Documentation Added:**
- F15 key choice explanation
- State file locations for all platforms
- Autostart mechanism details
- Project structure with descriptions
- Testing instructions
- Note that Tea is system tray-only

**Rationale:** Users and developers need comprehensive documentation.

**Impact:** Better onboarding for new users and contributors.

---

## Testing Results

### TypeScript Tests
```
✓ tests/autostart.test.ts (12 tests) 3ms
✓ tests/e2e.test.ts (7 tests) 19ms
✓ tests/frontend.test.ts (10 tests) 21ms

Test Files  3 passed (3)
     Tests  29 passed (29)
```

### Rust Tests
```
running 17 tests
test result: ok. 16 passed; 0 failed; 1 ignored
```

All tests passing ✅

---

## Migration Notes for Users

### State File Location Change (Windows Only)
**Old location:** `<executable_dir>/config/state.json`  
**New location:** `%LOCALAPPDATA%\tea\state.json`

**Action Required:** After updating, you'll need to:
1. Launch Tea
2. Reconfigure your preferences (sleep mode, screen control)
3. Re-enable "Start at Login" if desired

The old config file will remain in the old location but won't be used. You can safely delete it.

---

## Breaking Changes
- **Windows users**: State file location changed, requires one-time reconfiguration
- **Frontend**: Removed greet functionality (was never used in production)

---

## Non-Breaking Changes
- Documentation improvements
- Code cleanup
- Test suite improvements
- Mobile code removal (was not functional)

---

## Verification Checklist
- [x] All TypeScript tests pass
- [x] All Rust tests pass
- [x] Cargo check passes
- [x] Code compiles successfully
- [x] Documentation is accurate
- [x] No regressions in core functionality
