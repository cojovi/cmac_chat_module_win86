/**
 * Custom hook for keyboard shortcuts
 */

import { useEffect, useCallback } from 'react';

export type KeyboardModifier = 'ctrl' | 'alt' | 'shift' | 'meta';

export interface KeyboardShortcutOptions {
  key: string;
  modifiers?: KeyboardModifier[];
  enabled?: boolean;
  preventDefault?: boolean;
}

/**
 * Hook to handle keyboard shortcuts
 */
export function useKeyboardShortcut(
  callback: () => void,
  options: KeyboardShortcutOptions
) {
  const { key, modifiers = [], enabled = true, preventDefault = true } = options;

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!enabled) return;

      // Check if the key matches
      const keyMatches = event.key.toLowerCase() === key.toLowerCase();
      if (!keyMatches) return;

      // Check modifiers
      const ctrlMatch = modifiers.includes('ctrl') ? event.ctrlKey : !event.ctrlKey;
      const altMatch = modifiers.includes('alt') ? event.altKey : !event.altKey;
      const shiftMatch = modifiers.includes('shift') ? event.shiftKey : !event.shiftKey;
      const metaMatch = modifiers.includes('meta') ? event.metaKey : !event.metaKey;

      // Only trigger if all modifiers match
      if (ctrlMatch && altMatch && shiftMatch && metaMatch) {
        if (preventDefault) {
          event.preventDefault();
        }
        callback();
      }
    },
    [key, modifiers, enabled, preventDefault, callback]
  );

  useEffect(() => {
    if (!enabled) return;

    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, [enabled, handleKeyDown]);
}

/**
 * Hook for push-to-talk functionality
 * Typically uses spacebar or a modifier key
 */
export function usePushToTalk(
  onStart: () => void,
  onStop: () => void,
  options: Omit<KeyboardShortcutOptions, 'key'> & { key?: string } = {}
) {
  const { key = ' ', enabled = true, preventDefault = true, modifiers = [] } = options;

  const handleKeyDown = useCallback(
    (event: KeyboardEvent) => {
      if (!enabled) return;

      const keyMatches = event.key === key;
      if (!keyMatches) return;

      const ctrlMatch = modifiers.includes('ctrl') ? event.ctrlKey : !event.ctrlKey;
      const altMatch = modifiers.includes('alt') ? event.altKey : !event.altKey;
      const shiftMatch = modifiers.includes('shift') ? event.shiftKey : !event.shiftKey;
      const metaMatch = modifiers.includes('meta') ? event.metaKey : !event.metaKey;

      if (ctrlMatch && altMatch && shiftMatch && metaMatch) {
        if (preventDefault) {
          event.preventDefault();
        }
        onStart();
      }
    },
    [key, modifiers, enabled, preventDefault, onStart]
  );

  const handleKeyUp = useCallback(
    (event: KeyboardEvent) => {
      if (!enabled) return;

      const keyMatches = event.key === key;
      if (!keyMatches) return;

      if (preventDefault) {
        event.preventDefault();
      }
      onStop();
    },
    [key, enabled, preventDefault, onStop]
  );

  useEffect(() => {
    if (!enabled) return;

    window.addEventListener('keydown', handleKeyDown);
    window.addEventListener('keyup', handleKeyUp);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('keyup', handleKeyUp);
    };
  }, [enabled, handleKeyDown, handleKeyUp]);
}

/**
 * Format keyboard shortcut for display
 */
export function formatShortcut(
  key: string,
  modifiers: KeyboardModifier[] = []
): string {
  const isMac = navigator.platform.toUpperCase().indexOf('MAC') >= 0;
  const parts: string[] = [];

  if (modifiers.includes('ctrl')) {
    parts.push(isMac ? '⌃' : 'Ctrl');
  }
  if (modifiers.includes('alt')) {
    parts.push(isMac ? '⌥' : 'Alt');
  }
  if (modifiers.includes('shift')) {
    parts.push(isMac ? '⇧' : 'Shift');
  }
  if (modifiers.includes('meta')) {
    parts.push(isMac ? '⌘' : 'Win');
  }

  parts.push(key.toUpperCase());

  return parts.join(isMac ? '' : '+');
}
