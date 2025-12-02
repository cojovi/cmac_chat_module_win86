/**
 * Main App Component
 *
 * Root component for Talk to CMAC Voice Assistant
 */

import { ChatWindow } from './components/ChatWindow';
import './App.css';

function App() {
  return (
    <div className="app">
      <ChatWindow />
    </div>
  );
}

export default App;
