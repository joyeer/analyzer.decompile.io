import { useEffect, useState } from "react";
import "./App.css";
import { DragDropEvent, getCurrentWebview } from "@tauri-apps/api/webview";
import { Event } from "@tauri-apps/api/event";


function App() {
  
  useEffect(() => {
  
    const setupDragDrop = async () => {
      // 获取当前 Webview
      let webview = getCurrentWebview();
      const handler = (event: Event<DragDropEvent>) => {
        // event.detail 里通常包含拖拽的文件信息
        console.log("Native drag drop event:", event );
        
      };
      
      const unlisten = await webview.onDragDropEvent(handler);

      return () => {
        unlisten && unlisten();
      };
    };

    let cleanup: (() => void) | undefined;

    setupDragDrop().then((fn) => {
      cleanup = fn;
    });

    return () => {
      if (cleanup) cleanup();
    };
  }, []);


  return (
      <div>拖拽文件到窗口试试</div>
  );
}

export default App;

