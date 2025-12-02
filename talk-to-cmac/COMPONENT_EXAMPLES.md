# Component Examples & Code Snippets

Quick reference for implementing common UI patterns in Talk to CMAC.

## Table of Contents
- [Button Variants](#button-variants)
- [Input Patterns](#input-patterns)
- [Loading States](#loading-states)
- [Error Displays](#error-displays)
- [Modal/Dialog](#modaldialog)
- [Toast Notifications](#toast-notifications)
- [Empty States](#empty-states)

---

## Button Variants

### Primary Action Button
```typescript
// PrimaryButton.tsx
import React from 'react';
import styles from './PrimaryButton.module.css';

interface PrimaryButtonProps {
  children: React.ReactNode;
  onClick: () => void;
  disabled?: boolean;
  loading?: boolean;
}

export const PrimaryButton: React.FC<PrimaryButtonProps> = ({
  children,
  onClick,
  disabled = false,
  loading = false,
}) => {
  return (
    <button
      className={`${styles.button} ${loading ? styles.loading : ''}`}
      onClick={onClick}
      disabled={disabled || loading}
    >
      {loading ? (
        <>
          <span className={styles.spinner} />
          <span>Loading...</span>
        </>
      ) : (
        children
      )}
    </button>
  );
};
```

```css
/* PrimaryButton.module.css */
.button {
  padding: var(--spacing-md) var(--spacing-xl);
  border-radius: var(--radius-md);
  border: none;
  background: linear-gradient(135deg,
    var(--cmac-primary) 0%,
    var(--cmac-primary-light) 100%);
  color: white;
  font-size: var(--text-base);
  font-weight: var(--font-medium);
  font-family: var(--font-primary);
  cursor: pointer;
  transition: all var(--transition-base);
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--spacing-sm);
}

.button:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 8px 16px rgba(59, 130, 246, 0.3);
}

.button:active:not(:disabled) {
  transform: translateY(0);
}

.button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}
```

### Icon Button
```typescript
// IconButton.tsx
interface IconButtonProps {
  icon: React.ReactNode;
  onClick: () => void;
  ariaLabel: string;
  variant?: 'default' | 'danger';
}

export const IconButton: React.FC<IconButtonProps> = ({
  icon,
  onClick,
  ariaLabel,
  variant = 'default',
}) => {
  return (
    <button
      className={`icon-button ${variant}`}
      onClick={onClick}
      aria-label={ariaLabel}
    >
      {icon}
    </button>
  );
};
```

```css
.icon-button {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  border: none;
  background: var(--cmac-gray-200);
  color: var(--text-primary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-base);
}

.icon-button:hover {
  background: var(--cmac-gray-300);
  transform: scale(1.1);
}

.icon-button.danger {
  background: rgba(220, 38, 38, 0.1);
  color: var(--state-error);
}

.icon-button.danger:hover {
  background: rgba(220, 38, 38, 0.2);
}
```

---

## Input Patterns

### Text Input with Label
```typescript
// TextInput.tsx
interface TextInputProps {
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  error?: string;
  required?: boolean;
}

export const TextInput: React.FC<TextInputProps> = ({
  label,
  value,
  onChange,
  placeholder,
  error,
  required = false,
}) => {
  const id = React.useId();

  return (
    <div className="input-group">
      <label htmlFor={id} className="input-label">
        {label}
        {required && <span className="required">*</span>}
      </label>
      <input
        id={id}
        type="text"
        className={`input ${error ? 'error' : ''}`}
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder={placeholder}
        aria-invalid={!!error}
        aria-describedby={error ? `${id}-error` : undefined}
      />
      {error && (
        <span id={`${id}-error`} className="input-error" role="alert">
          {error}
        </span>
      )}
    </div>
  );
};
```

```css
.input-group {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-lg);
}

.input-label {
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  color: var(--text-primary);
}

.required {
  color: var(--state-error);
  margin-left: var(--spacing-xs);
}

.input {
  height: 40px;
  padding: 0 var(--spacing-md);
  border: 2px solid var(--border-color);
  border-radius: var(--radius-md);
  font-size: var(--text-base);
  font-family: var(--font-primary);
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: all var(--transition-base);
}

.input:focus {
  outline: none;
  border-color: var(--cmac-primary-light);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.input.error {
  border-color: var(--state-error);
}

.input.error:focus {
  box-shadow: 0 0 0 3px rgba(220, 38, 38, 0.1);
}

.input-error {
  font-size: var(--text-sm);
  color: var(--state-error);
  display: flex;
  align-items: center;
  gap: var(--spacing-xs);
}
```

### Search Input
```typescript
// SearchInput.tsx
export const SearchInput: React.FC<{
  value: string;
  onChange: (value: string) => void;
  onClear: () => void;
}> = ({ value, onChange, onClear }) => {
  return (
    <div className="search-input">
      <SearchIcon className="search-icon" />
      <input
        type="search"
        className="search-field"
        placeholder="Search conversations..."
        value={value}
        onChange={(e) => onChange(e.target.value)}
      />
      {value && (
        <button
          className="clear-button"
          onClick={onClear}
          aria-label="Clear search"
        >
          <XIcon />
        </button>
      )}
    </div>
  );
};
```

```css
.search-input {
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: var(--spacing-md);
  width: 20px;
  height: 20px;
  color: var(--text-secondary);
  pointer-events: none;
}

.search-field {
  width: 100%;
  height: 40px;
  padding: 0 var(--spacing-3xl) 0 var(--spacing-3xl);
  border: 2px solid var(--border-color);
  border-radius: var(--radius-lg);
  font-size: var(--text-base);
  background: var(--bg-primary);
  color: var(--text-primary);
}

.clear-button {
  position: absolute;
  right: var(--spacing-sm);
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: none;
  background: var(--cmac-gray-300);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}
```

---

## Loading States

### Skeleton Loader
```typescript
// Skeleton.tsx
export const Skeleton: React.FC<{
  width?: string;
  height?: string;
  rounded?: boolean;
}> = ({ width = '100%', height = '20px', rounded = false }) => {
  return (
    <div
      className={`skeleton ${rounded ? 'rounded' : ''}`}
      style={{ width, height }}
    />
  );
};

// Usage
export const MessageSkeleton = () => (
  <div className="message-skeleton">
    <Skeleton width="60%" height="16px" />
    <Skeleton width="80%" height="16px" />
    <Skeleton width="40%" height="12px" />
  </div>
);
```

```css
.skeleton {
  background: linear-gradient(
    90deg,
    var(--bg-secondary) 0%,
    var(--cmac-gray-200) 50%,
    var(--bg-secondary) 100%
  );
  background-size: 200% 100%;
  animation: shimmer 1.5s infinite;
  border-radius: var(--radius-sm);
}

.skeleton.rounded {
  border-radius: var(--radius-full);
}

.message-skeleton {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
}
```

### Spinner
```typescript
// Spinner.tsx
export const Spinner: React.FC<{
  size?: 'sm' | 'md' | 'lg';
  color?: string;
}> = ({ size = 'md', color }) => {
  const sizeMap = {
    sm: '16px',
    md: '24px',
    lg: '40px',
  };

  return (
    <div
      className="spinner"
      style={{
        width: sizeMap[size],
        height: sizeMap[size],
        borderColor: color || 'var(--cmac-primary-light)',
      }}
    />
  );
};
```

```css
.spinner {
  border: 3px solid rgba(59, 130, 246, 0.2);
  border-top-color: var(--cmac-primary-light);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}
```

### Loading Overlay
```typescript
// LoadingOverlay.tsx
export const LoadingOverlay: React.FC<{
  message?: string;
}> = ({ message = 'Loading...' }) => {
  return (
    <div className="loading-overlay">
      <div className="loading-content">
        <Spinner size="lg" />
        <p className="loading-message">{message}</p>
      </div>
    </div>
  );
};
```

```css
.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: var(--z-modal);
  animation: fadeIn 0.2s ease;
}

.loading-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-lg);
}

.loading-message {
  color: white;
  font-size: var(--text-lg);
  font-weight: var(--font-medium);
}
```

---

## Error Displays

### Inline Error
```typescript
// InlineError.tsx
export const InlineError: React.FC<{
  message: string;
  onRetry?: () => void;
}> = ({ message, onRetry }) => {
  return (
    <div className="inline-error" role="alert">
      <AlertIcon className="error-icon" />
      <span className="error-message">{message}</span>
      {onRetry && (
        <button className="retry-button" onClick={onRetry}>
          Retry
        </button>
      )}
    </div>
  );
};
```

```css
.inline-error {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-md);
  border-radius: var(--radius-md);
  background: rgba(220, 38, 38, 0.1);
  border: 1px solid var(--state-error);
  animation: slideInLeft 0.3s ease-out;
}

.error-icon {
  width: 20px;
  height: 20px;
  color: var(--state-error);
  flex-shrink: 0;
}

.error-message {
  flex: 1;
  font-size: var(--text-sm);
  color: var(--state-error);
}

.retry-button {
  padding: var(--spacing-xs) var(--spacing-md);
  border-radius: var(--radius-sm);
  border: 1px solid var(--state-error);
  background: transparent;
  color: var(--state-error);
  font-size: var(--text-sm);
  font-weight: var(--font-medium);
  cursor: pointer;
  transition: all var(--transition-base);
}

.retry-button:hover {
  background: var(--state-error);
  color: white;
}
```

### Error Banner
```typescript
// ErrorBanner.tsx
export const ErrorBanner: React.FC<{
  title: string;
  message: string;
  onDismiss: () => void;
}> = ({ title, message, onDismiss }) => {
  return (
    <div className="error-banner">
      <div className="error-content">
        <div className="error-header">
          <AlertIcon />
          <h3>{title}</h3>
        </div>
        <p>{message}</p>
      </div>
      <button
        className="dismiss-button"
        onClick={onDismiss}
        aria-label="Dismiss error"
      >
        <XIcon />
      </button>
    </div>
  );
};
```

```css
.error-banner {
  position: fixed;
  top: var(--spacing-lg);
  left: 50%;
  transform: translateX(-50%);
  max-width: 400px;
  width: calc(100% - var(--spacing-xl) * 2);
  padding: var(--spacing-lg);
  border-radius: var(--radius-lg);
  background: white;
  box-shadow: var(--shadow-xl);
  border-left: 4px solid var(--state-error);
  display: flex;
  gap: var(--spacing-md);
  animation: slideUp 0.3s ease-out;
  z-index: var(--z-popover);
}

.error-content {
  flex: 1;
}

.error-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  margin-bottom: var(--spacing-xs);
  color: var(--state-error);
}

.error-header h3 {
  margin: 0;
  font-size: var(--text-base);
  font-weight: var(--font-semibold);
}

.error-banner p {
  margin: 0;
  font-size: var(--text-sm);
  color: var(--text-secondary);
}

.dismiss-button {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: none;
  background: var(--cmac-gray-200);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}
```

---

## Modal/Dialog

### Base Modal
```typescript
// Modal.tsx
export const Modal: React.FC<{
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
}> = ({ isOpen, onClose, title, children }) => {
  if (!isOpen) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div
        className="modal-content"
        onClick={(e) => e.stopPropagation()}
        role="dialog"
        aria-modal="true"
        aria-labelledby="modal-title"
      >
        <div className="modal-header">
          <h2 id="modal-title">{title}</h2>
          <button
            className="close-button"
            onClick={onClose}
            aria-label="Close"
          >
            <XIcon />
          </button>
        </div>
        <div className="modal-body">{children}</div>
      </div>
    </div>
  );
};
```

```css
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: var(--z-modal);
  animation: fadeIn 0.2s ease;
}

.modal-content {
  max-width: 500px;
  width: calc(100% - var(--spacing-xl) * 2);
  max-height: 80vh;
  background: var(--bg-primary);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-2xl);
  overflow: hidden;
  animation: slideUp 0.3s ease-out;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-xl);
  border-bottom: 1px solid var(--border-color);
}

.modal-header h2 {
  margin: 0;
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}

.close-button {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: none;
  background: var(--cmac-gray-200);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-base);
}

.close-button:hover {
  background: var(--cmac-gray-300);
  transform: scale(1.1);
}

.modal-body {
  padding: var(--spacing-xl);
  overflow-y: auto;
}
```

---

## Toast Notifications

```typescript
// Toast.tsx
export const Toast: React.FC<{
  message: string;
  type?: 'success' | 'error' | 'info';
  onClose: () => void;
}> = ({ message, type = 'info', onClose }) => {
  React.useEffect(() => {
    const timer = setTimeout(onClose, 5000);
    return () => clearTimeout(timer);
  }, [onClose]);

  const icons = {
    success: <CheckIcon />,
    error: <AlertIcon />,
    info: <InfoIcon />,
  };

  return (
    <div className={`toast toast-${type}`}>
      <div className="toast-icon">{icons[type]}</div>
      <p className="toast-message">{message}</p>
      <button
        className="toast-close"
        onClick={onClose}
        aria-label="Close notification"
      >
        <XIcon />
      </button>
    </div>
  );
};
```

```css
.toast {
  position: fixed;
  bottom: var(--spacing-xl);
  right: var(--spacing-xl);
  min-width: 300px;
  max-width: 400px;
  padding: var(--spacing-lg);
  border-radius: var(--radius-lg);
  background: white;
  box-shadow: var(--shadow-xl);
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  animation: slideInRight 0.3s ease-out;
  z-index: var(--z-tooltip);
}

.toast-success {
  border-left: 4px solid var(--state-speaking);
}

.toast-error {
  border-left: 4px solid var(--state-error);
}

.toast-info {
  border-left: 4px solid var(--cmac-primary-light);
}

.toast-icon {
  width: 24px;
  height: 24px;
  flex-shrink: 0;
}

.toast-success .toast-icon {
  color: var(--state-speaking);
}

.toast-error .toast-icon {
  color: var(--state-error);
}

.toast-info .toast-icon {
  color: var(--cmac-primary-light);
}

.toast-message {
  flex: 1;
  margin: 0;
  font-size: var(--text-sm);
  color: var(--text-primary);
}

.toast-close {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: none;
  background: var(--cmac-gray-200);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}
```

---

## Empty States

```typescript
// EmptyState.tsx
export const EmptyState: React.FC<{
  icon: React.ReactNode;
  title: string;
  description: string;
  action?: {
    label: string;
    onClick: () => void;
  };
}> = ({ icon, title, description, action }) => {
  return (
    <div className="empty-state">
      <div className="empty-icon">{icon}</div>
      <h3 className="empty-title">{title}</h3>
      <p className="empty-description">{description}</p>
      {action && (
        <PrimaryButton onClick={action.onClick}>
          {action.label}
        </PrimaryButton>
      )}
    </div>
  );
};
```

```css
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: var(--spacing-3xl);
}

.empty-icon {
  width: 64px;
  height: 64px;
  margin-bottom: var(--spacing-lg);
  color: var(--text-secondary);
  opacity: 0.5;
}

.empty-title {
  margin: 0 0 var(--spacing-sm) 0;
  font-size: var(--text-xl);
  font-weight: var(--font-semibold);
  color: var(--text-primary);
}

.empty-description {
  margin: 0 0 var(--spacing-xl) 0;
  max-width: 300px;
  font-size: var(--text-base);
  color: var(--text-secondary);
  line-height: var(--leading-relaxed);
}
```

---

## Usage Examples

### Complete Settings Panel

```typescript
// Settings.tsx
export const Settings: React.FC<SettingsProps> = ({
  settings,
  isOpen,
  onClose,
  onSave,
}) => {
  const [localSettings, setLocalSettings] = React.useState(settings);
  const [testing, setTesting] = React.useState(false);

  const handleSave = async () => {
    onSave(localSettings);
    onClose();
  };

  return (
    <Modal isOpen={isOpen} onClose={onClose} title="Settings">
      <TextInput
        label="API Key"
        value={localSettings.apiKey}
        onChange={(value) =>
          setLocalSettings({ ...localSettings, apiKey: value })
        }
        placeholder="sk-..."
        required
      />

      <div className="settings-field">
        <label className="input-label">Voice Model</label>
        <select
          className="input"
          value={localSettings.voiceModel}
          onChange={(e) =>
            setLocalSettings({
              ...localSettings,
              voiceModel: e.target.value as VoiceModel,
            })
          }
        >
          {VOICE_MODELS.map((model) => (
            <option key={model.value} value={model.value}>
              {model.label}
            </option>
          ))}
        </select>
      </div>

      <div className="connection-status">
        {localSettings.connectionStatus.connected ? (
          <InlineError message="Connected" />
        ) : (
          <InlineError
            message="Not connected"
            onRetry={() => setTesting(true)}
          />
        )}
      </div>

      <div className="modal-actions">
        <button onClick={onClose}>Cancel</button>
        <PrimaryButton onClick={handleSave} loading={testing}>
          Save Settings
        </PrimaryButton>
      </div>
    </Modal>
  );
};
```

---

## Icon Components

```typescript
// icons.tsx
export const MicIcon = () => (
  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
    <path d="M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" />
    <path d="M19 10v2a7 7 0 0 1-14 0v-2" />
    <line x1="12" y1="19" x2="12" y2="23" />
  </svg>
);

export const AlertIcon = () => (
  <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
    <path d="M12 2a10 10 0 110 20 10 10 0 010-20zm0 11a1 1 0 100-2 1 1 0 000 2zm0-8a1 1 0 00-1 1v4a1 1 0 002 0V6a1 1 0 00-1-1z" />
  </svg>
);

export const CheckIcon = () => (
  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
    <polyline points="20 6 9 17 4 12" />
  </svg>
);

export const XIcon = () => (
  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor">
    <line x1="18" y1="6" x2="6" y2="18" />
    <line x1="6" y1="6" x2="18" y2="18" />
  </svg>
);

export const InfoIcon = () => (
  <svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
    <circle cx="12" cy="12" r="10" />
    <line x1="12" y1="16" x2="12" y2="12" stroke="white" strokeWidth="2" />
    <circle cx="12" cy="8" r="1" fill="white" />
  </svg>
);
```

---

**Quick Reference Card**

```
Component Checklist:
☐ Accessible (ARIA labels, keyboard nav)
☐ Responsive (works at min/max dimensions)
☐ Dark mode support
☐ Loading states
☐ Error states
☐ Focus indicators
☐ Smooth animations (60fps)
☐ TypeScript types
☐ CSS modules (scoped styles)
☐ Reduced motion support
```
