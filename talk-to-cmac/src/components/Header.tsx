/**
 * Header Component
 *
 * App header with status and controls
 */

import { memo } from 'react';
import { StatusIndicator } from './StatusIndicator';
import { ConnectionStatus } from './ConnectionStatus';
import type { AppStatus, ConnectivityStatus } from '../types';
import './Header.css';

interface HeaderProps {
  status: AppStatus;
  connectivity: ConnectivityStatus | null;
  onSettingsClick: () => void;
  onClearChat: () => void;
  onRefreshConnectivity: () => void;
}

export const Header = memo(function Header({
  status,
  connectivity,
  onSettingsClick,
  onClearChat,
  onRefreshConnectivity,
}: HeaderProps) {
  return (
    <header className="app-header">
      <div className="header-left">
        <div className="app-title">
          <h1>Talk to CMAC</h1>
        </div>
        <StatusIndicator status={status} />
      </div>

      <div className="header-center">
        {connectivity && <ConnectionStatus connectivity={connectivity} />}
      </div>

      <div className="header-right">
        <button
          className="icon-button"
          onClick={onRefreshConnectivity}
          aria-label="Refresh connectivity"
          title="Refresh connectivity"
        >
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 1 18.8-4.3M22 12.5a10 10 0 0 1-18.8 4.2" />
          </svg>
        </button>

        <button
          className="icon-button"
          onClick={onClearChat}
          aria-label="Clear conversation"
          title="Clear conversation"
        >
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <path d="M3 6h18" />
            <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
            <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
          </svg>
        </button>

        <button
          className="icon-button"
          onClick={onSettingsClick}
          aria-label="Open settings"
          title="Settings"
        >
          <svg
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <circle cx="12" cy="12" r="3" />
            <path d="M12 1v6M12 17v6M4.22 4.22l4.24 4.24M15.54 15.54l4.24 4.24M1 12h6M17 12h6M4.22 19.78l4.24-4.24M15.54 8.46l4.24-4.24" />
          </svg>
        </button>
      </div>
    </header>
  );
});
