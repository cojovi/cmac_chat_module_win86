/**
 * Microphone Button Component
 *
 * Push-to-talk button with visual feedback
 */

import { memo } from 'react';
import type { MicrophoneButtonProps } from '../types';
import './MicrophoneButton.css';

export const MicrophoneButton = memo(function MicrophoneButton({
  isRecording,
  isProcessing,
  onStartRecording,
  onStopRecording,
  disabled = false,
}: MicrophoneButtonProps) {
  const handleClick = () => {
    if (isRecording) {
      onStopRecording();
    } else {
      onStartRecording();
    }
  };

  const getButtonClass = () => {
    const classes = ['microphone-button'];
    if (isRecording) classes.push('recording');
    if (isProcessing) classes.push('processing');
    if (disabled) classes.push('disabled');
    return classes.join(' ');
  };

  const getButtonText = () => {
    if (isProcessing) return 'Processing...';
    if (isRecording) return 'Stop Recording';
    return 'Hold to Talk';
  };

  const getAriaLabel = () => {
    if (isProcessing) return 'Processing audio';
    if (isRecording) return 'Stop recording';
    return 'Start recording';
  };

  return (
    <button
      className={getButtonClass()}
      onClick={handleClick}
      disabled={disabled || isProcessing}
      aria-label={getAriaLabel()}
      aria-pressed={isRecording}
    >
      <div className="mic-icon-wrapper">
        <svg
          className="mic-icon"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
        >
          <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />
          <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
          <line x1="12" y1="19" x2="12" y2="23" />
          <line x1="8" y1="23" x2="16" y2="23" />
        </svg>
        {isRecording && (
          <div className="recording-rings">
            <div className="ring ring-1" />
            <div className="ring ring-2" />
            <div className="ring ring-3" />
          </div>
        )}
      </div>
      <span className="button-text">{getButtonText()}</span>
    </button>
  );
});
