/**
 * Message Bubble Component
 *
 * Individual chat message display
 */

import { memo } from 'react';
import type { MessageBubbleProps } from '../types';
import './MessageBubble.css';

export const MessageBubble = memo(function MessageBubble({
  message,
  isUser,
  onPlayAudio,
}: MessageBubbleProps) {
  const timestamp = new Date(message.timestamp).toLocaleTimeString([], {
    hour: '2-digit',
    minute: '2-digit',
  });

  const handlePlayAudio = () => {
    if (message.audioData && onPlayAudio) {
      onPlayAudio(message.audioData);
    }
  };

  return (
    <div className={`message-bubble ${isUser ? 'user' : 'assistant'}`}>
      <div className="message-header">
        <span className="message-role">
          {isUser ? 'You' : 'CMAC'}
        </span>
        <span className="message-timestamp">{timestamp}</span>
      </div>
      <div className="message-content">
        {message.content}
      </div>
      {message.audioData && !isUser && (
        <button
          className="audio-play-button"
          onClick={handlePlayAudio}
          aria-label="Play audio response"
        >
          <svg
            viewBox="0 0 24 24"
            fill="currentColor"
            width="16"
            height="16"
          >
            <path d="M8 5v14l11-7z" />
          </svg>
          <span>Play Audio</span>
        </button>
      )}
    </div>
  );
});
