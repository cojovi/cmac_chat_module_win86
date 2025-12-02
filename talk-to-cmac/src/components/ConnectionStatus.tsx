/**
 * Connection Status Component
 *
 * Displays connectivity status for all services
 */

import { memo } from 'react';
import type { ConnectionStatusProps, ServiceStatus } from '../types';
import { isServiceConnected, getServiceErrorMessage } from '../utils/tauri';
import './ConnectionStatus.css';

export const ConnectionStatus = memo(function ConnectionStatus({
  connectivity,
}: ConnectionStatusProps) {
  if (!connectivity) {
    return null;
  }

  const getStatusIcon = (status: ServiceStatus): string => {
    if (isServiceConnected(status)) {
      return '✓';
    }
    if (status === 'checking') {
      return '⟳';
    }
    return '✗';
  };

  const getStatusClass = (status: ServiceStatus): string => {
    if (isServiceConnected(status)) {
      return 'connected';
    }
    if (status === 'checking') {
      return 'checking';
    }
    return 'disconnected';
  };

  const renderService = (name: string, status: ServiceStatus) => {
    const errorMessage = getServiceErrorMessage(status);

    return (
      <div
        key={name}
        className={`service-status service-${getStatusClass(status)}`}
        title={errorMessage || undefined}
      >
        <span className="service-icon" aria-hidden="true">
          {getStatusIcon(status)}
        </span>
        <span className="service-name">{name}</span>
      </div>
    );
  };

  const lastChecked = new Date(connectivity.last_checked).toLocaleTimeString();

  return (
    <div className="connection-status">
      <div className="services">
        {renderService('Whisper', connectivity.whisper)}
        {renderService('OpenWebUI', connectivity.openwebui)}
        {renderService('ElevenLabs', connectivity.elevenlabs)}
      </div>
      <div className="last-checked" title="Last checked">
        {lastChecked}
      </div>
    </div>
  );
});
