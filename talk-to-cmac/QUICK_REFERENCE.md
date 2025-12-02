# Talk to CMAC - Quick Reference Card

**Print this page and keep it handy while working**

---

## Essential Colors (Copy-Paste Ready)

```css
/* Primary Brand */
--cmac-primary: #1E3A8A;
--cmac-primary-light: #3B82F6;
--cmac-secondary: #DC2626;

/* State Colors */
--state-listening: #3B82F6;    /* Blue - recording */
--state-transcribing: #F59E0B; /* Yellow - processing */
--state-thinking: #8B5CF6;     /* Purple - AI thinking */
--state-speaking: #10B981;     /* Green - TTS playing */
--state-error: #DC2626;        /* Red - error */
--state-offline: #64748B;      /* Gray - offline */

/* Light Mode */
--bg-primary: #FFFFFF;
--text-primary: #0F172A;
--border-color: #E2E8F0;

/* Dark Mode */
--bg-primary: #0F172A;
--text-primary: #F8FAFC;
--border-color: #334155;
```

---

## Typography Scale

```css
--text-xs: 0.75rem;   /* 12px - timestamps */
--text-sm: 0.875rem;  /* 14px - labels */
--text-base: 1rem;    /* 16px - body */
--text-lg: 1.125rem;  /* 18px - title */
--text-xl: 1.25rem;   /* 20px - headers */
```

---

## Spacing Scale

```css
--spacing-xs: 4px;
--spacing-sm: 8px;
--spacing-md: 12px;
--spacing-lg: 16px;   /* Most common */
--spacing-xl: 24px;
--spacing-2xl: 32px;
```

---

## Component Dimensions

| Component | Width | Height | Notes |
|-----------|-------|--------|-------|
| Main Window | 420px | 650px | Min: 360×400 |
| Status Bar | 100% | 48px | Fixed |
| Header | 100% | 80px | Fixed |
| Mic Button | 64px | 64px | Circle |
| Text Input | Flex | 48px | Pill shape |
| Input Area | 100% | 72px | Fixed |

---

## Border Radius

```css
--radius-sm: 4px;
--radius-md: 8px;
--radius-lg: 12px;
--radius-xl: 16px;    /* Main window */
--radius-2xl: 24px;   /* Text input */
--radius-full: 9999px; /* Buttons */
```

---

## Shadows

```css
/* Buttons */
box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);

/* Panels */
box-shadow: 0 10px 15px rgba(0, 0, 0, 0.1);

/* Main Window */
box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
```

---

## Common Animations

```css
/* Pulse (listening) */
animation: pulse 2s ease-in-out infinite;

/* Spin (loading) */
animation: spin 1s linear infinite;

/* Slide In */
animation: slideInRight 0.3s ease-out; /* User message */
animation: slideInLeft 0.3s ease-out;  /* AI message */

/* Fade In */
animation: fadeIn 0.2s ease;
```

---

## Transition Speeds

```css
--transition-fast: 150ms ease;  /* Quick feedback */
--transition-base: 200ms ease;  /* Standard */
--transition-slow: 300ms ease;  /* Smooth */
```

---

## State Order

```
INITIALIZING → IDLE → LISTENING → TRANSCRIBING
→ THINKING → SPEAKING → IDLE
```

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Ctrl+M | Toggle microphone |
| Ctrl+S | Open settings |
| Ctrl+L | Clear conversation |
| Escape | Close modals |
| Enter | Send message |
| Tab | Navigate forward |
| Shift+Tab | Navigate backward |

---

## Common CSS Patterns

### Glassmorphism Effect
```css
background: rgba(255, 255, 255, 0.85);
backdrop-filter: blur(20px);
border: 1px solid rgba(255, 255, 255, 0.5);
```

### Focus Indicator
```css
*:focus-visible {
  outline: 3px solid var(--cmac-primary-light);
  outline-offset: 2px;
}
```

### Pulse Animation
```css
@keyframes pulse {
  0%, 100% { transform: scale(1); opacity: 1; }
  50% { transform: scale(1.05); opacity: 0.8; }
}
```

---

## Quick TypeScript Types

```typescript
// Application State
type AppState =
  | 'idle' | 'listening' | 'transcribing'
  | 'thinking' | 'speaking' | 'error';

// Message Type
type MessageType = 'user' | 'assistant' | 'system';

// Theme Mode
type ThemeMode = 'light' | 'dark' | 'auto';
```

---

## Common Component Props

```typescript
// Button
interface ButtonProps {
  onClick: () => void;
  disabled?: boolean;
  children: React.ReactNode;
}

// Input
interface InputProps {
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  error?: string;
}

// Message
interface MessageProps {
  content: string;
  type: 'user' | 'assistant';
  timestamp: Date;
  audioUrl?: string;
}
```

---

## File Structure

```
/src
├── styles/
│   ├── variables.css      ← Import first
│   └── animations.css     ← Import second
├── types/
│   └── design-system.ts   ← Type definitions
├── components/
│   ├── StatusBar/
│   ├── MicButton/
│   ├── MessageBubble/
│   └── Settings/
├── hooks/
│   └── useTheme.ts        ← Theme management
└── store/
    └── appStore.ts        ← State management
```

---

## CSS Import Order

```typescript
// In main.tsx
import './styles/variables.css';   // 1. Design tokens
import './styles/animations.css';  // 2. Animations
import App from './App';            // 3. Components
```

---

## Accessibility Checklist

- [ ] All interactive elements keyboard accessible
- [ ] Focus indicators visible
- [ ] ARIA labels on buttons
- [ ] Color contrast 7:1+ (WCAG AAA)
- [ ] Reduced motion support
- [ ] Screen reader tested
- [ ] Semantic HTML used

---

## Common Mistakes to Avoid

❌ **Don't:**
- Use hard-coded colors (use CSS variables)
- Forget dark mode styles
- Skip accessibility attributes
- Use `left`/`width` for animations (use `transform`)
- Forget focus indicators
- Hard-code dimensions (use design tokens)

✅ **Do:**
- Use CSS variables for all colors
- Test in both light/dark modes
- Add ARIA labels
- Use `transform` and `opacity` for animations
- Test keyboard navigation
- Use spacing/sizing variables

---

## Testing Quick Checks

### Visual
- [ ] Matches [VISUAL_REFERENCE.md](/Users/cojovi/dev/windows_gpt/talk-to-cmac/VISUAL_REFERENCE.md)
- [ ] Light mode works
- [ ] Dark mode works
- [ ] Animations smooth (60fps)

### Interaction
- [ ] Buttons respond to clicks
- [ ] Keyboard navigation works
- [ ] Focus indicators visible
- [ ] States transition correctly

### Accessibility
- [ ] Tab order logical
- [ ] Screen reader announces correctly
- [ ] Color contrast sufficient
- [ ] Works with reduced motion

---

## Where to Find...

| Need | Document | Section |
|------|----------|---------|
| Color hex codes | DESIGN_SYSTEM.md | Section 1 |
| Component code | IMPLEMENTATION_GUIDE.md | Components |
| Exact measurements | VISUAL_REFERENCE.md | Throughout |
| Animation specs | animations.css | All keyframes |
| Type definitions | design-system.ts | All types |
| Usage patterns | COMPONENT_EXAMPLES.md | All sections |

---

## Emergency Debugging

### Issue: Colors not showing
→ Check CSS variables imported in `main.tsx`

### Issue: Dark mode not working
→ Verify `data-theme` attribute on `<html>`

### Issue: Animations choppy
→ Use `transform` and `opacity` only
→ Add `will-change` sparingly

### Issue: Keyboard nav broken
→ Check `tabindex` and `:focus-visible` styles

### Issue: Layout misaligned
→ Compare with VISUAL_REFERENCE.md measurements

---

## Version Info

**Design System**: v1.0
**Created**: 2025-12-01
**Files**: 5 markdown docs + 3 code files

---

## Quick Commands

```bash
# Install dependencies
npm install

# Run dev server
npm run dev

# Build for production
npm run build

# Run Tauri dev
npm run tauri dev
```

---

**Keep this card handy while working on Talk to CMAC!**

For complete details, see:
- [DESIGN_INDEX.md](/Users/cojovi/dev/windows_gpt/talk-to-cmac/DESIGN_INDEX.md) - Navigation guide
- [DESIGN_SYSTEM.md](/Users/cojovi/dev/windows_gpt/talk-to-cmac/DESIGN_SYSTEM.md) - Complete specs
- [IMPLEMENTATION_GUIDE.md](/Users/cojovi/dev/windows_gpt/talk-to-cmac/IMPLEMENTATION_GUIDE.md) - Development guide

---

**Print-Friendly Version**: This page is formatted to print on 2-3 pages
