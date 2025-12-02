/**
 * Error Message Component
 *
 * Displays error messages with dismiss functionality
 */

import { memo, useEffect } from 'react';
import type { ErrorMessageProps } from '../types';
import './ErrorMessage.css';

export const ErrorMessage = memo(function ErrorMessage({
  error,
  onDismiss,
}: ErrorMessageProps) {
  // Auto-dismiss after 10 seconds
  useEffect(() => {
    if (!error) return;

    const timer = setTimeout(() => {
      onDismiss();
    }, 10000);

    return () => clearTimeout(timer);
  }, [error, onDismiss]);

  if (!error) {
    return null;
  }

  const timeAgo = Date.now() - error.timestamp;
  const seconds = Math.floor(timeAgo / 1000);
  const timeText = seconds < 60 ? `${seconds}s ago` : `${Math.floor(seconds / 60)}m ago`;

  return (
    <div className="error-message" role="alert">
      <div className="error-content">
        <span className="error-icon" aria-hidden="true">
          ⚠
        </span>
        <div className="error-details">
          <div className="error-text">{error.message}</div>
          {error.source && (
            <div className="error-source">
              {error.source} • {timeText}
            </div>
          )}
        </div>
      </div>
      <button
        className="error-dismiss"
        onClick={onDismiss}
        aria-label="Dismiss error"
      >
        ✕
      </button>
    </div>
  );
});
