/**
 * Status Indicator Component
 *
 * Shows current app status with animations
 */

import { memo } from 'react';
import type { StatusIndicatorProps } from '../types';
import './StatusIndicator.css';

export const StatusIndicator = memo(function StatusIndicator({
  status,
}: StatusIndicatorProps) {
  const getStatusText = (): string => {
    if (typeof status === 'string') {
      switch (status) {
        case 'idle':
          return 'Ready';
        case 'recording':
          return 'Recording...';
        case 'listening':
          return 'Listening...';
        case 'transcribing':
          return 'Transcribing...';
        case 'thinking':
          return 'Thinking...';
        case 'speaking':
          return 'Speaking...';
        default:
          return 'Ready';
      }
    }
    // Error status
    return 'Error';
  };

  const getStatusClass = (): string => {
    if (typeof status === 'string') {
      return status;
    }
    return 'error';
  };

  const getStatusIcon = (): string => {
    if (typeof status === 'string') {
      switch (status) {
        case 'idle':
          return '◉';
        case 'recording':
          return '●';
        case 'listening':
          return '◉';
        case 'transcribing':
          return '⟳';
        case 'thinking':
          return '●●●';
        case 'speaking':
          return '♪';
        default:
          return '◉';
      }
    }
    return '⚠';
  };

  return (
    <div className={`status-indicator status-${getStatusClass()}`}>
      <span className="status-icon" aria-hidden="true">
        {getStatusIcon()}
      </span>
      <span className="status-text">{getStatusText()}</span>
    </div>
  );
});
