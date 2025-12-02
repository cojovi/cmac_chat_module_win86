/**
 * Tauri Command Wrappers
 *
 * Type-safe wrapper functions for all 13 Tauri backend commands
 */

import { invoke } from '@tauri-apps/api/core';
import type {
  AppConfig,
  AppStateResponse,
  ApiService,
  ConversationContext,
  ConnectivityResponse,
  Voice,
  VoiceSettings,
  VoiceQueryResponse,
} from '../types';

// ============================================================================
// Audio Processing Commands
// ============================================================================

/**
 * Transcribe audio to text using Whisper API
 */
export async function processAudio(
  audioData: Uint8Array,
  filename: string = 'recording.wav'
): Promise<string> {
  try {
    return await invoke<string>('process_audio', {
      audioData: Array.from(audioData),
      filename,
    });
  } catch (error) {
    throw new Error(`Audio transcription failed: ${error}`);
  }
}

/**
 * Convert text to speech using ElevenLabs API
 */
export async function synthesizeSpeech(text: string): Promise<Uint8Array> {
  try {
    if (text.length > 5000) {
      throw new Error('Text exceeds maximum length of 5000 characters');
    }

    const audioBytes = await invoke<number[]>('synthesize_speech', { text });
    return new Uint8Array(audioBytes);
  } catch (error) {
    throw new Error(`Speech synthesis failed: ${error}`);
  }
}

// ============================================================================
// LLM Interaction Commands
// ============================================================================

/**
 * Send a text message to the LLM and get a response
 */
export async function sendMessage(message: string): Promise<string> {
  try {
    return await invoke<string>('send_message', { message });
  } catch (error) {
    throw new Error(`Failed to send message: ${error}`);
  }
}

// ============================================================================
// Complete Pipeline Commands
// ============================================================================

/**
 * Process complete voice assistant pipeline (audio → text → LLM → speech)
 */
export async function processVoiceQuery(
  audioData: Uint8Array,
  filename: string = 'recording.wav'
): Promise<VoiceQueryResponse> {
  try {
    return await invoke<VoiceQueryResponse>('process_voice_query', {
      audioData: Array.from(audioData),
      filename,
    });
  } catch (error) {
    throw new Error(`Voice query processing failed: ${error}`);
  }
}

// ============================================================================
// Configuration Management Commands
// ============================================================================

/**
 * Load current application configuration
 */
export async function loadConfig(): Promise<AppConfig> {
  try {
    return await invoke<AppConfig>('load_config');
  } catch (error) {
    throw new Error(`Failed to load configuration: ${error}`);
  }
}

/**
 * Save application configuration
 */
export async function saveConfig(config: AppConfig): Promise<void> {
  try {
    await invoke('save_config', { config });
  } catch (error) {
    throw new Error(`Failed to save configuration: ${error}`);
  }
}

/**
 * Store an API key securely in the system keyring
 */
export async function updateApiKey(
  service: ApiService,
  apiKey: string
): Promise<void> {
  try {
    await invoke('update_api_key', { service, apiKey });
  } catch (error) {
    throw new Error(`Failed to update ${service} API key: ${error}`);
  }
}

// ============================================================================
// State Management Commands
// ============================================================================

/**
 * Get current application state
 */
export async function getAppState(): Promise<AppStateResponse> {
  try {
    return await invoke<AppStateResponse>('get_app_state');
  } catch (error) {
    throw new Error(`Failed to get app state: ${error}`);
  }
}

/**
 * Get conversation history
 */
export async function getConversation(): Promise<ConversationContext> {
  try {
    return await invoke<ConversationContext>('get_conversation');
  } catch (error) {
    throw new Error(`Failed to get conversation: ${error}`);
  }
}

/**
 * Clear conversation history and start fresh
 */
export async function clearConversation(): Promise<void> {
  try {
    await invoke('clear_conversation');
  } catch (error) {
    throw new Error(`Failed to clear conversation: ${error}`);
  }
}

// ============================================================================
// Connectivity Commands
// ============================================================================

/**
 * Check connectivity to all services
 */
export async function checkConnectivity(): Promise<ConnectivityResponse> {
  try {
    return await invoke<ConnectivityResponse>('check_connectivity');
  } catch (error) {
    throw new Error(`Failed to check connectivity: ${error}`);
  }
}

// ============================================================================
// Voice Management Commands
// ============================================================================

/**
 * List available ElevenLabs voices
 */
export async function listVoices(): Promise<Voice[]> {
  try {
    return await invoke<Voice[]>('list_voices');
  } catch (error) {
    throw new Error(`Failed to list voices: ${error}`);
  }
}

/**
 * Update ElevenLabs voice synthesis settings
 */
export async function updateVoiceSettings(
  settings: VoiceSettings
): Promise<void> {
  try {
    // Validate settings
    if (settings.stability < 0 || settings.stability > 1) {
      throw new Error('Stability must be between 0 and 1');
    }
    if (settings.similarity_boost < 0 || settings.similarity_boost > 1) {
      throw new Error('Similarity boost must be between 0 and 1');
    }
    if (settings.style !== undefined && (settings.style < 0 || settings.style > 1)) {
      throw new Error('Style must be between 0 and 1');
    }

    await invoke('update_voice_settings', { settings });
  } catch (error) {
    throw new Error(`Failed to update voice settings: ${error}`);
  }
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Check if a service status indicates connection
 */
export function isServiceConnected(status: any): boolean {
  return status === 'connected';
}

/**
 * Extract error message from service status
 */
export function getServiceErrorMessage(status: any): string | null {
  if (typeof status === 'object' && 'disconnected' in status) {
    return status.disconnected.reason;
  }
  return null;
}

/**
 * Check if app status is an error
 */
export function isErrorStatus(status: any): boolean {
  return typeof status === 'object' && 'error' in status;
}

/**
 * Extract error message from app status
 */
export function getErrorMessage(status: any): string | null {
  if (isErrorStatus(status)) {
    return status.error.message;
  }
  return null;
}
