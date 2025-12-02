/**
 * Zustand Store for Talk to CMAC Voice Assistant
 *
 * Manages global application state including:
 * - App status and processing states
 * - Conversation history
 * - Configuration
 * - Connectivity status
 * - Error handling
 */

import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import type {
  AppStatus,
  AppConfig,
  Message,
  ErrorState,
  ConnectivityStatus,
  ThemeMode,
} from '../types';

interface AppState {
  // Status
  status: AppStatus;
  isProcessing: boolean;
  isRecording: boolean;

  // Conversation
  messages: Message[];
  conversationId: string | null;

  // Configuration
  config: AppConfig | null;

  // Connectivity
  connectivity: ConnectivityStatus | null;

  // Error handling
  error: ErrorState | null;

  // UI State
  theme: ThemeMode;
  isSidebarOpen: boolean;
  isSettingsOpen: boolean;

  // Actions - Status
  setStatus: (status: AppStatus) => void;
  setProcessing: (isProcessing: boolean) => void;
  setRecording: (isRecording: boolean) => void;

  // Actions - Conversation
  addMessage: (message: Message) => void;
  setMessages: (messages: Message[]) => void;
  clearMessages: () => void;
  setConversationId: (id: string | null) => void;
  updateLastMessage: (content: string) => void;

  // Actions - Configuration
  setConfig: (config: AppConfig) => void;
  updateConfig: (partial: Partial<AppConfig>) => void;

  // Actions - Connectivity
  setConnectivity: (connectivity: ConnectivityStatus) => void;

  // Actions - Error
  setError: (error: ErrorState | null) => void;
  clearError: () => void;

  // Actions - UI
  setTheme: (theme: ThemeMode) => void;
  toggleSidebar: () => void;
  toggleSettings: () => void;
  setSettingsOpen: (isOpen: boolean) => void;

  // Actions - Reset
  reset: () => void;
}

const initialState = {
  status: 'idle' as AppStatus,
  isProcessing: false,
  isRecording: false,
  messages: [] as Message[],
  conversationId: null,
  config: null,
  connectivity: null,
  error: null,
  theme: 'auto' as ThemeMode,
  isSidebarOpen: false,
  isSettingsOpen: false,
};

export const useAppStore = create<AppState>()(
  devtools(
    persist(
      (set, get) => ({
        ...initialState,

        // Status Actions
        setStatus: (status) => {
          set({ status }, false, 'setStatus');

          // Automatically clear error if returning to idle
          if (status === 'idle' && get().error) {
            set({ error: null }, false, 'clearError');
          }

          // Track processing state based on status
          const processingStates = ['transcribing', 'thinking', 'speaking'];
          const isProcessing = typeof status === 'string' && processingStates.includes(status);
          set({ isProcessing }, false, 'updateProcessing');
        },

        setProcessing: (isProcessing) =>
          set({ isProcessing }, false, 'setProcessing'),

        setRecording: (isRecording) => {
          set({ isRecording }, false, 'setRecording');
          if (isRecording) {
            set({ status: 'recording' }, false, 'statusRecording');
          } else if (get().status === 'recording') {
            set({ status: 'idle' }, false, 'statusIdle');
          }
        },

        // Conversation Actions
        addMessage: (message) =>
          set(
            (state) => ({ messages: [...state.messages, message] }),
            false,
            'addMessage'
          ),

        setMessages: (messages) =>
          set({ messages }, false, 'setMessages'),

        clearMessages: () =>
          set({ messages: [], conversationId: null }, false, 'clearMessages'),

        setConversationId: (id) =>
          set({ conversationId: id }, false, 'setConversationId'),

        updateLastMessage: (content) =>
          set(
            (state) => {
              const messages = [...state.messages];
              if (messages.length > 0) {
                messages[messages.length - 1] = {
                  ...messages[messages.length - 1],
                  content,
                };
              }
              return { messages };
            },
            false,
            'updateLastMessage'
          ),

        // Configuration Actions
        setConfig: (config) =>
          set({ config }, false, 'setConfig'),

        updateConfig: (partial) =>
          set(
            (state) => ({
              config: state.config ? { ...state.config, ...partial } : null,
            }),
            false,
            'updateConfig'
          ),

        // Connectivity Actions
        setConnectivity: (connectivity) =>
          set({ connectivity }, false, 'setConnectivity'),

        // Error Actions
        setError: (error) => {
          set({ error }, false, 'setError');

          // Automatically set status to error if error is set
          if (error) {
            set({
              status: { error: { message: error.message } },
              isProcessing: false
            }, false, 'statusError');
          }
        },

        clearError: () =>
          set({ error: null }, false, 'clearError'),

        // UI Actions
        setTheme: (theme) =>
          set({ theme }, false, 'setTheme'),

        toggleSidebar: () =>
          set((state) => ({ isSidebarOpen: !state.isSidebarOpen }), false, 'toggleSidebar'),

        toggleSettings: () =>
          set((state) => ({ isSettingsOpen: !state.isSettingsOpen }), false, 'toggleSettings'),

        setSettingsOpen: (isOpen) =>
          set({ isSettingsOpen: isOpen }, false, 'setSettingsOpen'),

        // Reset Actions
        reset: () =>
          set(initialState, false, 'reset'),
      }),
      {
        name: 'talk-to-cmac-storage',
        partialize: (state) => ({
          theme: state.theme,
          // Only persist UI preferences, not conversation data
        }),
      }
    )
  )
);

// Selectors for optimized component rendering
export const selectStatus = (state: AppState) => state.status;
export const selectMessages = (state: AppState) => state.messages;
export const selectLastMessage = (state: AppState) =>
  state.messages[state.messages.length - 1];
export const selectIsProcessing = (state: AppState) => state.isProcessing;
export const selectIsRecording = (state: AppState) => state.isRecording;
export const selectError = (state: AppState) => state.error;
export const selectConnectivity = (state: AppState) => state.connectivity;
export const selectConfig = (state: AppState) => state.config;
export const selectTheme = (state: AppState) => state.theme;
