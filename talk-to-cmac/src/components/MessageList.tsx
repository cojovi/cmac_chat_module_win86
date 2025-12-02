/**
 * Message List Component
 *
 * Scrollable list of conversation messages
 */

import { memo, useRef, useEffect } from 'react';
import { MessageBubble } from './MessageBubble';
import type { Message } from '../types';
import './MessageList.css';

interface MessageListProps {
  messages: Message[];
  onPlayAudio?: (audioData: Uint8Array) => void;
}

export const MessageList = memo(function MessageList({
  messages,
  onPlayAudio,
}: MessageListProps) {
  const scrollRef = useRef<HTMLDivElement>(null);
  const lastMessageRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom when new messages arrive
  useEffect(() => {
    if (lastMessageRef.current) {
      lastMessageRef.current.scrollIntoView({
        behavior: 'smooth',
        block: 'end',
      });
    }
  }, [messages.length]);

  if (messages.length === 0) {
    return (
      <div className="message-list-empty">
        <div className="empty-state">
          <svg
            className="empty-icon"
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
          <h2>Welcome to Talk to CMAC</h2>
          <p>Press and hold the microphone to start a conversation</p>
        </div>
      </div>
    );
  }

  // Filter out system messages for display
  const displayMessages = messages.filter((msg) => msg.role !== 'system');

  return (
    <div className="message-list" ref={scrollRef}>
      <div className="messages-container">
        {displayMessages.map((message, index) => (
          <div
            key={`${message.timestamp}-${index}`}
            ref={index === displayMessages.length - 1 ? lastMessageRef : null}
          >
            <MessageBubble
              message={message}
              isUser={message.role === 'user'}
              onPlayAudio={onPlayAudio}
            />
          </div>
        ))}
      </div>
    </div>
  );
});
