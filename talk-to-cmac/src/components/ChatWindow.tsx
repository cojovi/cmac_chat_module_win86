/**
 * Chat Window Component
 *
 * Main chat interface container
 */

import { memo, useCallback, useEffect } from 'react';
import { useAppStore, selectMessages, selectStatus, selectError, selectConnectivity } from '../store/useAppStore';
import { useTauri } from '../hooks/useTauri';
import { useAudioRecorder } from '../hooks/useAudioRecorder';
import { useAudioPlayer } from '../hooks/useAudioPlayer';
import { Header } from './Header';
import { MessageList } from './MessageList';
import { InputArea } from './InputArea';
import { ErrorMessage } from './ErrorMessage';
import type { Message } from '../types';
import './ChatWindow.css';

export const ChatWindow = memo(function ChatWindow() {
  // Store state
  const messages = useAppStore(selectMessages);
  const status = useAppStore(selectStatus);
  const error = useAppStore(selectError);
  const connectivity = useAppStore(selectConnectivity);
  const { addMessage, setError, clearError, setStatus, setSettingsOpen } = useAppStore();

  // Tauri commands
  const {
    sendTextMessage,
    processVoiceQuery,
    clearConversation,
    checkConnectivity,
  } = useTauri();

  // Audio hooks
  const recorder = useAudioRecorder({
    sampleRate: 16000,
    channels: 1,
    bitsPerSample: 16,
    maxDuration: 60000,
  });

  const player = useAudioPlayer();

  // Handle recording start
  const handleStartRecording = useCallback(async () => {
    try {
      await recorder.startRecording();
    } catch (err) {
      setError({
        message: err instanceof Error ? err.message : 'Failed to start recording',
        timestamp: Date.now(),
        source: 'microphone',
      });
    }
  }, [recorder, setError]);

  // Handle recording stop
  const handleStopRecording = useCallback(async () => {
    try {
      const audioData = await recorder.stopRecording();
      if (!audioData) {
        throw new Error('No audio data recorded');
      }

      // Add user message placeholder
      const userMessage: Message = {
        role: 'user',
        content: 'Processing...',
        timestamp: Date.now(),
      };
      addMessage(userMessage);

      // Process voice query
      setStatus('transcribing');
      const result = await processVoiceQuery(audioData);

      // Update user message with transcription
      const transcribedMessage: Message = {
        role: 'user',
        content: result.transcription,
        timestamp: Date.now(),
      };
      addMessage(transcribedMessage);

      // Add assistant response
      const assistantMessage: Message = {
        role: 'assistant',
        content: result.llm_response,
        timestamp: Date.now(),
        audioData: new Uint8Array(result.audio_response),
      };
      addMessage(assistantMessage);

      // Play audio response
      setStatus('speaking');
      await player.play(new Uint8Array(result.audio_response));
      setStatus('idle');
    } catch (err) {
      setError({
        message: err instanceof Error ? err.message : 'Failed to process voice query',
        timestamp: Date.now(),
        source: 'voice-processing',
      });
      setStatus('idle');
    }
  }, [recorder, addMessage, setStatus, setError, processVoiceQuery, player]);

  // Handle text message send
  const handleSendMessage = useCallback(async (message: string) => {
    try {
      // Add user message
      const userMessage: Message = {
        role: 'user',
        content: message,
        timestamp: Date.now(),
      };
      addMessage(userMessage);

      // Get LLM response
      setStatus('thinking');
      const response = await sendTextMessage(message);

      // Add assistant response
      const assistantMessage: Message = {
        role: 'assistant',
        content: response,
        timestamp: Date.now(),
      };
      addMessage(assistantMessage);

      setStatus('idle');
    } catch (err) {
      setError({
        message: err instanceof Error ? err.message : 'Failed to send message',
        timestamp: Date.now(),
        source: 'text-message',
      });
      setStatus('idle');
    }
  }, [addMessage, setStatus, setError, sendTextMessage]);

  // Handle clear chat
  const handleClearChat = useCallback(async () => {
    try {
      await clearConversation();
    } catch (err) {
      setError({
        message: err instanceof Error ? err.message : 'Failed to clear conversation',
        timestamp: Date.now(),
        source: 'clear-chat',
      });
    }
  }, [clearConversation, setError]);

  // Handle refresh connectivity
  const handleRefreshConnectivity = useCallback(async () => {
    try {
      await checkConnectivity();
    } catch (err) {
      console.error('Failed to check connectivity:', err);
    }
  }, [checkConnectivity]);

  // Handle settings click
  const handleSettingsClick = useCallback(() => {
    setSettingsOpen(true);
  }, [setSettingsOpen]);

  // Handle play audio
  const handlePlayAudio = useCallback(async (audioData: Uint8Array) => {
    try {
      await player.play(audioData);
    } catch (err) {
      setError({
        message: err instanceof Error ? err.message : 'Failed to play audio',
        timestamp: Date.now(),
        source: 'audio-playback',
      });
    }
  }, [player, setError]);

  // Show audio player errors
  useEffect(() => {
    if (player.error) {
      setError({
        message: player.error,
        timestamp: Date.now(),
        source: 'audio-player',
      });
    }
  }, [player.error, setError]);

  // Show audio recorder errors
  useEffect(() => {
    if (recorder.error) {
      setError({
        message: recorder.error,
        timestamp: Date.now(),
        source: 'audio-recorder',
      });
    }
  }, [recorder.error, setError]);

  const isProcessing = status !== 'idle' && status !== 'recording';

  return (
    <div className="chat-window">
      <Header
        status={status}
        connectivity={connectivity}
        onSettingsClick={handleSettingsClick}
        onClearChat={handleClearChat}
        onRefreshConnectivity={handleRefreshConnectivity}
      />

      <div className="chat-content">
        {error && <ErrorMessage error={error} onDismiss={clearError} />}
        <MessageList messages={messages} onPlayAudio={handlePlayAudio} />
      </div>

      <InputArea
        onSendMessage={handleSendMessage}
        onStartRecording={handleStartRecording}
        onStopRecording={handleStopRecording}
        isRecording={recorder.isRecording}
        isProcessing={isProcessing}
      />
    </div>
  );
});
