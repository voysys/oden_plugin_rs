'use client';
import { useEffect, useRef, useState } from 'react';
import OdenVideo from './odenVideo';
import { getOrCreateOdenLayoutClient } from './lib/oden';

export default function MultiCamLayout() {
  const containerRef = useRef<HTMLDivElement>(null);
  const debugRef = useRef<HTMLDivElement>(null);

  const [lastIncoming, setLastIncoming] = useState<{ payload: any } | null>(null);

  // Register inbound message handler once on mount
  useEffect(() => {
    const layoutClient = getOrCreateOdenLayoutClient();
    const incomingHandler = (payload: any) => {
      console.info('[Oden] Incoming user message', JSON.stringify(payload));
      setLastIncoming({ payload });
    };

    layoutClient.registerUserMessageCallback("test_message", incomingHandler);

    return () => {
      if (typeof layoutClient.unregisterUserMessageCallback === 'function') {
        layoutClient.unregisterUserMessageCallback("test_message", incomingHandler);
      }
    };
  }, []);

  // Outbound test message
  const handleSendClick = () => {
    const layoutClient = getOrCreateOdenLayoutClient();
    layoutClient.sendNamedUserMessage('ui:demoButton', {
      clicked: true,
      ts: Date.now(),
    });
  };

  return (
    <div
      ref={containerRef}
      className="relative grid grid-cols-3 gap-2"
    >
      {/* Left Camera */}
      <div className="relative">
        <OdenVideo name="left_cam" />
        <div className="absolute top-2 left-2 bg-black/60 text-white text-sm px-2 py-1 rounded">
          Left Camera
        </div>
      </div>

      {/* Front Camera + Rear Camera Overlay */}
      <div className="relative">
        <OdenVideo name="front_cam" />
        <div className="absolute top-2 left-2 bg-black/60 text-white text-sm px-2 py-1 rounded">
          Front Camera
        </div>

        <div className="absolute top-4 right-4 w-1/4">
          <div className="relative">
            <OdenVideo name="rear_cam" z="1" scale_x="-1.0" />
            <div className="absolute top-2 left-2 bg-black/60 text-white text-sm px-2 py-1 rounded">
              Rear Camera
            </div>
          </div>
        </div>
      </div>

      {/* Right Camera */}
      <div className="relative">
        <OdenVideo name="right_cam" />
        <div className="absolute top-2 left-2 bg-black/60 text-white text-sm px-2 py-1 rounded">
          Right Camera
        </div>
      </div>

      {/* Floating action button */}
      <button
        onClick={handleSendClick}
        className="fixed bottom-4 right-4 z-50 rounded-2xl bg-blue-600 px-4 py-2 text-white shadow-lg hover:bg-blue-700 active:scale-95"
      >
        Send Oden Message
      </button>

      {/* Debug overlay */}
      <div className="pointer-events-none bottom-4 left-4 max-w-md rounded bg-black/60 p-3 text-xs text-lime-300 backdrop-blur">
        {lastIncoming ? JSON.stringify(lastIncoming, null, 2) : 'Awaiting incoming messages…'}
      </div>
    </div>
  );
}
