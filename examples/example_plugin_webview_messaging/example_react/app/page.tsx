'use client';
import { useEffect, useState } from 'react';
import OdenVideo from './odenVideo';
import { getOrCreateOdenLayoutClient } from './lib/oden';

export default function MultiCamLayout() {
  const [lastIncoming, setLastIncoming] = useState<{ payload: any } | null>(null);

  // Register inbound message handler once on mount
  useEffect(() => {
    const layoutClient = getOrCreateOdenLayoutClient();
    const incomingHandler = (payload: any) => {
      console.info('[Oden] Incoming user message', JSON.stringify(payload));
      setLastIncoming({ payload });
    };

    layoutClient.registerUserMessageCallback("example_to_webview", incomingHandler);

    return () => {
      if (typeof layoutClient.unregisterUserMessageCallback === 'function') {
        layoutClient.unregisterUserMessageCallback("example_to_webview", incomingHandler);
      }
    };
  }, []);

  // Outbound test message
  const handleSendClick = () => {
    const layoutClient = getOrCreateOdenLayoutClient();
    layoutClient.sendNamedUserMessage('example_from_webview', { ts: Date.now() });
  };

  return (
    <div className="camera-grid">
      {/* Left Camera */}
      <div className="camera">
        <OdenVideo name="left_cam" />
        <div className="camera-label">Left Camera</div>
      </div>

      {/* Front Camera + Rear Camera Overlay */}
      <div className="camera">
        <OdenVideo name="front_cam" />
        <div className="camera-label">Front Camera</div>

        <div className="camera-inset">
          <div className="camera">
            <OdenVideo name="rear_cam" z="1" scale_x="-1.0" />
            <div className="camera-label">Rear Camera</div>
          </div>
        </div>
      </div>

      {/* Right Camera */}
      <div className="camera">
        <OdenVideo name="right_cam" />
        <div className="camera-label">Right Camera</div>
      </div>

      {/* Floating action button */}
      <button onClick={handleSendClick} className="send-button">
        Send example_from_webview
      </button>

      {/* Debug overlay */}
      <div className="debug-overlay">
        {lastIncoming ? JSON.stringify(lastIncoming, null, 2) : 'Awaiting incoming messages…'}
      </div>
    </div>
  );
}
