/**
 * Custom hook for audio playback
 */

import { useState, useCallback, useRef, useEffect } from 'react';
import { AudioPlayer } from '../utils/audio';

export interface UseAudioPlayerResult {
  isPlaying: boolean;
  currentTime: number;
  duration: number;
  error: string | null;
  play: (audioData: Uint8Array) => Promise<void>;
  stop: () => void;
  pause: () => void;
  resume: () => void;
  resetError: () => void;
}

export function useAudioPlayer(): UseAudioPlayerResult {
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);
  const [error, setError] = useState<string | null>(null);

  const playerRef = useRef<AudioPlayer>(new AudioPlayer());
  const progressTimerRef = useRef<number | null>(null);

  // Clear progress timer
  const clearProgressTimer = useCallback(() => {
    if (progressTimerRef.current) {
      clearInterval(progressTimerRef.current);
      progressTimerRef.current = null;
    }
  }, []);

  // Start progress timer
  const startProgressTimer = useCallback(() => {
    clearProgressTimer();
    progressTimerRef.current = setInterval(() => {
      if (playerRef.current.isPlaying) {
        setCurrentTime(playerRef.current.currentTime);
        setDuration(playerRef.current.duration);
      }
    }, 100);
  }, [clearProgressTimer]);

  // Play audio
  const play = useCallback(async (audioData: Uint8Array) => {
    try {
      setError(null);
      setIsPlaying(true);
      setCurrentTime(0);
      setDuration(0);

      startProgressTimer();

      await playerRef.current.play(audioData);

      // Playback ended
      setIsPlaying(false);
      setCurrentTime(0);
      setDuration(0);
      clearProgressTimer();
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to play audio';
      setError(errorMessage);
      setIsPlaying(false);
      clearProgressTimer();
    }
  }, [startProgressTimer, clearProgressTimer]);

  // Stop playback
  const stop = useCallback(() => {
    playerRef.current.stop();
    setIsPlaying(false);
    setCurrentTime(0);
    setDuration(0);
    clearProgressTimer();
  }, [clearProgressTimer]);

  // Pause playback
  const pause = useCallback(() => {
    if (isPlaying) {
      playerRef.current.pause();
      setIsPlaying(false);
      clearProgressTimer();
    }
  }, [isPlaying, clearProgressTimer]);

  // Resume playback
  const resume = useCallback(() => {
    if (!isPlaying) {
      playerRef.current.resume();
      setIsPlaying(true);
      startProgressTimer();
    }
  }, [isPlaying, startProgressTimer]);

  // Reset error
  const resetError = useCallback(() => {
    setError(null);
  }, []);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      clearProgressTimer();
      playerRef.current.stop();
    };
  }, [clearProgressTimer]);

  return {
    isPlaying,
    currentTime,
    duration,
    error,
    play,
    stop,
    pause,
    resume,
    resetError,
  };
}
