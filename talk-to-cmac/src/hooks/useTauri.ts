/**
 * Custom hook for Tauri integration
 */

import { useCallback, useEffect } from 'react';
import { useAppStore } from '../store/useAppStore';
import * as tauri from '../utils/tauri';
import type { AppConfig, VoiceSettings, ApiService } from '../types';

export function useTauri() {
  const {
    setConfig,
    setMessages,
    setConversationId,
    setConnectivity,
    setStatus,
    setError,
  } = useAppStore();

  // Initialize app on mount
  useEffect(() => {
    const initialize = async () => {
      try {
        // Load configuration
        const config = await tauri.loadConfig();
        setConfig(config);

        // Get app state
        const state = await tauri.getAppState();
        setStatus(state.status);
        setConnectivity(state.connectivity);

        // Load conversation history
        const conversation = await tauri.getConversation();
        setConversationId(conversation.id);
        setMessages(conversation.messages);

        // Check connectivity
        await checkConnectivity();
      } catch (error) {
        console.error('Failed to initialize app:', error);
        setError({
          message: error instanceof Error ? error.message : 'Initialization failed',
          timestamp: Date.now(),
          source: 'initialization',
        });
      }
    };

    initialize();
  }, [setConfig, setMessages, setConversationId, setConnectivity, setStatus, setError]);

  // Check connectivity
  const checkConnectivity = useCallback(async () => {
    try {
      const connectivity = await tauri.checkConnectivity();
      setConnectivity({
        ...connectivity,
        last_checked: Date.now(),
      });
    } catch (error) {
      console.error('Failed to check connectivity:', error);
    }
  }, [setConnectivity]);

  // Send text message
  const sendTextMessage = useCallback(
    async (message: string) => {
      try {
        setStatus('thinking');
        const response = await tauri.sendMessage(message);
        setStatus('idle');
        return response;
      } catch (error) {
        setStatus('idle');
        throw error;
      }
    },
    [setStatus]
  );

  // Process voice query
  const processVoiceQuery = useCallback(
    async (audioData: Uint8Array) => {
      try {
        const result = await tauri.processVoiceQuery(audioData);
        return result;
      } catch (error) {
        throw error;
      }
    },
    []
  );

  // Save configuration
  const saveConfiguration = useCallback(
    async (config: AppConfig) => {
      try {
        await tauri.saveConfig(config);
        setConfig(config);
      } catch (error) {
        throw error;
      }
    },
    [setConfig]
  );

  // Update API key
  const updateApiKey = useCallback(
    async (service: ApiService, apiKey: string) => {
      try {
        await tauri.updateApiKey(service, apiKey);
      } catch (error) {
        throw error;
      }
    },
    []
  );

  // Clear conversation
  const clearConversation = useCallback(async () => {
    try {
      await tauri.clearConversation();
      setMessages([]);
      setConversationId(null);
    } catch (error) {
      throw error;
    }
  }, [setMessages, setConversationId]);

  // List voices
  const listVoices = useCallback(async () => {
    try {
      return await tauri.listVoices();
    } catch (error) {
      throw error;
    }
  }, []);

  // Update voice settings
  const updateVoiceSettings = useCallback(async (settings: VoiceSettings) => {
    try {
      await tauri.updateVoiceSettings(settings);
    } catch (error) {
      throw error;
    }
  }, []);

  return {
    checkConnectivity,
    sendTextMessage,
    processVoiceQuery,
    saveConfiguration,
    updateApiKey,
    clearConversation,
    listVoices,
    updateVoiceSettings,
  };
}
