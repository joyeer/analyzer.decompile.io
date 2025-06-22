import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DragDropEvent, getCurrentWebview } from "@tauri-apps/api/webview";
import { Event } from "@tauri-apps/api/event";
import OpenProject from "./components/welcome";
import "./App.css";


function App() {
  
  const [projectId, setProjectId] = useState<string | null>(null);
  
  const handleOpenFile = useCallback( async (path: String) => {

    console.log("Opening project at path:", path);
    try {
      const projectId = await invoke<string>("create_project", { path });
      setProjectId(projectId);
    }
    catch (error) {
      console.error("Failed to open project:", error);
      return;
    }
  }, []);

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
    <div
      className="min-h-screen flex flex-col items-center justify-center bg-gray-50"
      onDragOver={e => e.preventDefault()}
      >
        <OpenProject onOpen={handleOpenFile}/>
      </div>
  );
}

export default App;

