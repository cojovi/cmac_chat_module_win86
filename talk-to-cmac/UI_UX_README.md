# Talk to CMAC - UI/UX Design Documentation

Complete design system and implementation guides for the Talk to CMAC voice assistant.

## Overview

Talk to CMAC is a Windows desktop voice assistant with a Siri-inspired aesthetic, designed specifically for CMAC Roofing. This documentation provides everything needed to implement a professional, accessible, and delightful user interface.

## Design Philosophy

### Core Principles
1. **User-Centered**: Every decision prioritizes the end user's needs
2. **Professional yet Approachable**: Reflects CMAC's construction industry expertise
3. **Accessible First**: WCAG AAA compliance built-in from the start
4. **Performance Optimized**: 60fps animations, minimal bundle size
5. **Consistent**: Design system ensures cohesive experience

### Visual Language
- **Glassmorphism**: Modern, translucent layers inspired by macOS Big Sur
- **Smooth Animations**: Purposeful motion that guides attention
- **State-Driven Design**: Clear visual feedback for every system state
- **Brand Integration**: CMAC colors and identity throughout

---

## Documentation Structure

### 1. DESIGN_SYSTEM.md (Main Reference)
**Complete design specifications**
- Color palette with hex codes
- Typography system
- Component specifications with exact dimensions
- State diagram and interaction flows
- Animation specifications
- Accessibility guidelines
- Branding integration

**Use this for**: Understanding the complete design system, making design decisions

### 2. IMPLEMENTATION_GUIDE.md (Developer Guide)
**Step-by-step implementation instructions**
- Quick start setup
- Component implementation order
- Code examples with TypeScript
- State management with Zustand
- Testing checklist
- Week-by-week implementation roadmap

**Use this for**: Building the application, following best practices

### 3. COMPONENT_EXAMPLES.md (Code Library)
**Ready-to-use component patterns**
- Button variants
- Input patterns
- Loading states
- Error displays
- Modal/dialog
- Toast notifications
- Empty states
- Icon library

**Use this for**: Copy-paste implementations, common UI patterns

### 4. Design Assets

#### CSS Files
- `/src/styles/variables.css` - All CSS custom properties
- `/src/styles/animations.css` - Keyframe animations

#### TypeScript
- `/src/types/design-system.ts` - Type definitions for all components

---

## Quick Start Guide

### For Designers

1. **Review the Design System**
   ```
   Read: DESIGN_SYSTEM.md
   Focus: Sections 1-3 (Colors, Typography, Components)
   ```

2. **Understand User Flows**
   ```
   Read: DESIGN_SYSTEM.md → Section 6 (Interaction Flows)
   Review: State diagram (Section 5)
   ```

3. **Check Accessibility**
   ```
   Read: DESIGN_SYSTEM.md → Section 7 (Accessibility)
   Ensure: All designs meet WCAG AAA standards
   ```

### For Developers

1. **Set Up the Project**
   ```bash
   # Install dependencies
   npm install

   # Import design system styles in main.tsx
   import './styles/variables.css';
   import './styles/animations.css';
   ```

2. **Follow Implementation Order**
   ```
   Read: IMPLEMENTATION_GUIDE.md
   Week 1: Foundation components (StatusBar, MicButton, MessageBubble)
   Week 2: State management and interactions
   Week 3: Settings and error handling
   Week 4: Polish and testing
   ```

3. **Use Component Examples**
   ```
   Read: COMPONENT_EXAMPLES.md
   Copy: Ready-to-use patterns
   Customize: Adapt to specific needs
   ```

### For Product Managers

1. **Understand Scope**
   ```
   Read: This file (overview) + DESIGN_SYSTEM.md sections 1, 5, 6
   Understand: User flows, states, error handling
   ```

2. **Review Timeline**
   ```
   Read: IMPLEMENTATION_GUIDE.md → Implementation Priority
   4-week implementation plan with clear milestones
   ```

---

## Key Features

### Visual States
The app supports 8 distinct states with unique visual feedback:
- **Initializing**: Loading on startup
- **Idle**: Ready for input
- **Listening**: Recording voice input
- **Transcribing**: Converting speech to text
- **Thinking**: AI processing
- **Speaking**: Playing TTS response
- **Error**: Something went wrong
- **Offline**: No API connection

### Interaction Methods
1. **Voice Input**: Push-to-talk via microphone button
2. **Text Input**: Traditional text field with Enter to send
3. **Keyboard Shortcuts**: Full keyboard navigation support

### Design Highlights
- **420x650px compact window** - Fits comfortably on any screen
- **Glassmorphism effects** - Modern, translucent aesthetic
- **Smooth 60fps animations** - Pulse, spin, slide, fade
- **Light/Dark mode** - Automatic system preference detection
- **Accessible** - WCAG AAA compliant, screen reader support

---

## Color Palette at a Glance

```
Primary Brand Colors:
--cmac-primary: #1E3A8A (Deep Blue)
--cmac-secondary: #DC2626 (Bold Red)

State Colors:
--state-listening: #3B82F6 (Blue)
--state-transcribing: #F59E0B (Yellow)
--state-thinking: #8B5CF6 (Purple)
--state-speaking: #10B981 (Green)
--state-error: #DC2626 (Red)
--state-offline: #64748B (Gray)
```

---

## Component Hierarchy

```
App (Main Container)
├── StatusBar
│   ├── StatusIndicator (state icon + label)
│   └── SettingsButton
├── Header
│   ├── Logo
│   ├── Title ("Talk to CMAC")
│   └── Subtitle ("Your Roofing Assistant")
├── MessageList (Scrollable)
│   └── MessageBubble[]
│       ├── Content
│       ├── AudioControls (optional)
│       └── Timestamp
├── InputArea
│   ├── TextInput
│   └── MicButton
└── Settings (Modal)
    ├── SettingsPanel
    │   ├── API Key Input
    │   ├── Voice Selection
    │   ├── Hotkey Config
    │   └── Connection Status
    └── Overlay
```

---

## Technical Stack

### Frontend
- **React 19**: UI framework
- **TypeScript**: Type safety
- **Zustand**: State management
- **CSS Modules**: Scoped styling

### Desktop Integration
- **Tauri 2**: Native app wrapper
- **System Tray**: Background presence
- **Global Shortcuts**: Keyboard hotkeys

### Design System
- **CSS Custom Properties**: Design tokens
- **Keyframe Animations**: Smooth transitions
- **Responsive Design**: Flexible layouts

---

## Browser/Platform Support

### Primary Target
- Windows 10 (version 1809+)
- Windows 11

### Tested Resolutions
- 1920x1080 (Full HD)
- 2560x1440 (2K)
- 3840x2160 (4K)
- Various DPI scaling (100%, 125%, 150%)

### Accessibility
- Screen readers: NVDA, JAWS
- High contrast mode
- Reduced motion support
- Keyboard-only navigation

---

## File Reference

### Design Documentation
```
/DESIGN_SYSTEM.md          - Complete design specifications
/IMPLEMENTATION_GUIDE.md   - Developer implementation guide
/COMPONENT_EXAMPLES.md     - Code snippets and patterns
/UI_UX_README.md          - This file (overview)
```

### Code Files
```
/src/styles/
  variables.css           - CSS custom properties
  animations.css          - Keyframe animations

/src/types/
  design-system.ts        - TypeScript type definitions

/src/components/
  StatusBar/
  MicButton/
  MessageBubble/
  Settings/
  (etc.)

/src/hooks/
  useTheme.ts            - Theme management hook

/src/store/
  appStore.ts            - Zustand state management
```

---

## Implementation Checklist

### Phase 1: Foundation (Week 1)
- [ ] Set up CSS variables system
- [ ] Create base component structure
- [ ] Implement StatusBar
- [ ] Build MicButton with states
- [ ] Create MessageBubble (user & assistant)
- [ ] Add light/dark mode toggle

### Phase 2: Interactions (Week 2)
- [ ] Add all animations (pulse, spin, slide, fade)
- [ ] Wire up state management (Zustand)
- [ ] Implement keyboard navigation
- [ ] Add focus indicators
- [ ] Build text input with send
- [ ] Create audio playback controls

### Phase 3: Advanced (Week 3)
- [ ] Settings panel with validation
- [ ] Error state handling
- [ ] System tray integration
- [ ] Window positioning logic
- [ ] Accessibility audit
- [ ] Performance optimization

### Phase 4: Polish (Week 4)
- [ ] Fine-tune animations
- [ ] Add loading states
- [ ] Design empty states
- [ ] Create onboarding flow
- [ ] Keyboard shortcuts help
- [ ] Cross-platform testing

---

## Design Decisions & Rationale

### Why Glassmorphism?
- Modern aesthetic that stands out from traditional desktop apps
- Creates visual hierarchy through layers and depth
- Aligns with Siri-inspired design goal
- Professional yet contemporary feel

### Why Push-to-Talk (No Hotword)?
- Better privacy (no always-listening)
- More reliable activation
- Prevents false triggers
- Industry standard for desktop voice apps

### Why 420x650px Window?
- Compact enough to not obstruct work
- Large enough for comfortable reading
- Fits standard laptop screens (1366x768+)
- Matches typical messenger app dimensions

### Why CSS Modules over Tailwind?
- Better for complex animations
- Clearer separation of concerns
- Easier to maintain custom brand styles
- No build-time processing overhead
- More familiar to designers

---

## Accessibility Commitment

This design system prioritizes accessibility:

- **WCAG AAA**: 7:1+ contrast ratios for all text
- **Keyboard First**: Full navigation without mouse
- **Screen Reader**: Semantic HTML and ARIA labels
- **Reduced Motion**: Respects user preferences
- **Focus Indicators**: Always visible when navigating
- **Error Recovery**: Clear messages with retry options

---

## Getting Help

### Design Questions
Refer to: `DESIGN_SYSTEM.md`
Sections: Colors, Typography, Component Specifications

### Implementation Questions
Refer to: `IMPLEMENTATION_GUIDE.md`
Sections: Component Implementation, State Management

### Code Examples
Refer to: `COMPONENT_EXAMPLES.md`
Find: Ready-to-use patterns and snippets

### Troubleshooting
1. Check the testing checklist in IMPLEMENTATION_GUIDE.md
2. Review accessibility guidelines in DESIGN_SYSTEM.md
3. Verify CSS variables are imported correctly
4. Ensure TypeScript types are up to date

---

## Version History

**v1.0** (2025-12-01)
- Initial design system creation
- Complete component specifications
- Implementation guide with code examples
- Accessibility-first approach
- 4-week implementation roadmap

---

## Contributing

When adding new components:
1. Follow existing patterns in COMPONENT_EXAMPLES.md
2. Use CSS variables from variables.css
3. Include TypeScript types
4. Add accessibility attributes
5. Test in light/dark modes
6. Support reduced motion
7. Document usage examples

---

## License & Credits

**Design System**: Created for CMAC Roofing
**Designer**: Claude (AI UI/UX Specialist)
**Project**: Talk to CMAC Voice Assistant
**Date**: December 2025

---

## Next Steps

1. **For the team**: Review DESIGN_SYSTEM.md together
2. **For designers**: Create high-fidelity mockups based on specs
3. **For developers**: Follow IMPLEMENTATION_GUIDE.md week-by-week
4. **For testers**: Use testing checklist for QA validation

**Ready to build?** Start with IMPLEMENTATION_GUIDE.md → Quick Start section.

---

**Questions or feedback?** All documentation is version-controlled and can be updated as the design evolves.
