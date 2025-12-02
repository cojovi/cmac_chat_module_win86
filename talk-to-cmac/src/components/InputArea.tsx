/**
 * Input Area Component
 *
 * Text input with microphone button
 */

import { memo, useState, useCallback, KeyboardEvent } from 'react';
import { MicrophoneButton } from './MicrophoneButton';
import './InputArea.css';

interface InputAreaProps {
  onSendMessage: (message: string) => void;
  onStartRecording: () => void;
  onStopRecording: () => void;
  isRecording: boolean;
  isProcessing: boolean;
  disabled?: boolean;
}

export const InputArea = memo(function InputArea({
  onSendMessage,
  onStartRecording,
  onStopRecording,
  isRecording,
  isProcessing,
  disabled = false,
}: InputAreaProps) {
  const [message, setMessage] = useState('');

  const handleSend = useCallback(() => {
    const trimmedMessage = message.trim();
    if (trimmedMessage && !isProcessing) {
      onSendMessage(trimmedMessage);
      setMessage('');
    }
  }, [message, isProcessing, onSendMessage]);

  const handleKeyDown = useCallback(
    (e: KeyboardEvent<HTMLTextAreaElement>) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        handleSend();
      }
    },
    [handleSend]
  );

  return (
    <div className="input-area">
      <div className="input-container">
        <textarea
          className="message-input"
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Type a message or use voice..."
          disabled={disabled || isProcessing}
          rows={1}
          maxLength={1000}
          aria-label="Message input"
        />
        <button
          className="send-button"
          onClick={handleSend}
          disabled={!message.trim() || isProcessing || disabled}
          aria-label="Send message"
        >
          <svg
            viewBox="0 0 24 24"
            fill="currentColor"
            width="20"
            height="20"
          >
            <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z" />
          </svg>
        </button>
      </div>
      <div className="mic-container">
        <MicrophoneButton
          isRecording={isRecording}
          isProcessing={isProcessing}
          onStartRecording={onStartRecording}
          onStopRecording={onStopRecording}
          disabled={disabled}
        />
      </div>
    </div>
  );
});
