/**
 * Custom hook for audio recording
 */

import { useState, useCallback, useRef, useEffect } from 'react';
import { createAudioRecorder, type AudioRecorder, type RecordingOptions } from '../utils/audio';

export interface UseAudioRecorderResult {
  isRecording: boolean;
  isPaused: boolean;
  duration: number;
  error: string | null;
  startRecording: () => Promise<void>;
  stopRecording: () => Promise<Uint8Array | null>;
  pauseRecording: () => void;
  resumeRecording: () => void;
  resetError: () => void;
}

export function useAudioRecorder(
  options: RecordingOptions = {}
): UseAudioRecorderResult {
  const [isRecording, setIsRecording] = useState(false);
  const [isPaused, setIsPaused] = useState(false);
  const [duration, setDuration] = useState(0);
  const [error, setError] = useState<string | null>(null);

  const recorderRef = useRef<AudioRecorder | null>(null);
  const durationTimerRef = useRef<number | null>(null);

  // Clear duration timer
  const clearDurationTimer = useCallback(() => {
    if (durationTimerRef.current) {
      clearInterval(durationTimerRef.current);
      durationTimerRef.current = null;
    }
  }, []);

  // Start duration timer
  const startDurationTimer = useCallback(() => {
    clearDurationTimer();
    durationTimerRef.current = setInterval(() => {
      if (recorderRef.current && !recorderRef.current.isPaused) {
        setDuration(Math.floor(recorderRef.current.duration / 1000));
      }
    }, 100);
  }, [clearDurationTimer]);

  // Start recording
  const startRecording = useCallback(async () => {
    try {
      setError(null);
      setDuration(0);

      recorderRef.current = await createAudioRecorder(options);
      await recorderRef.current.start();

      setIsRecording(true);
      setIsPaused(false);
      startDurationTimer();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to start recording';
      setError(errorMessage);
      setIsRecording(false);
      recorderRef.current = null;
    }
  }, [options, startDurationTimer]);

  // Stop recording
  const stopRecording = useCallback(async (): Promise<Uint8Array | null> => {
    if (!recorderRef.current) {
      return null;
    }

    try {
      clearDurationTimer();
      const audioData = await recorderRef.current.stop();

      setIsRecording(false);
      setIsPaused(false);
      setDuration(0);
      recorderRef.current = null;

      return audioData;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to stop recording';
      setError(errorMessage);
      setIsRecording(false);
      setIsPaused(false);
      recorderRef.current = null;
      return null;
    }
  }, [clearDurationTimer]);

  // Pause recording
  const pauseRecording = useCallback(() => {
    if (recorderRef.current && isRecording && !isPaused) {
      recorderRef.current.pause();
      setIsPaused(true);
      clearDurationTimer();
    }
  }, [isRecording, isPaused, clearDurationTimer]);

  // Resume recording
  const resumeRecording = useCallback(() => {
    if (recorderRef.current && isRecording && isPaused) {
      recorderRef.current.resume();
      setIsPaused(false);
      startDurationTimer();
    }
  }, [isRecording, isPaused, startDurationTimer]);

  // Reset error
  const resetError = useCallback(() => {
    setError(null);
  }, []);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      clearDurationTimer();
      if (recorderRef.current) {
        recorderRef.current.stop().catch(() => {
          // Ignore errors during cleanup
        });
      }
    };
  }, [clearDurationTimer]);

  return {
    isRecording,
    isPaused,
    duration,
    error,
    startRecording,
    stopRecording,
    pauseRecording,
    resumeRecording,
    resetError,
  };
}
