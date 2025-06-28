import { useCallback, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { DragDropEvent, getCurrentWebview } from "@tauri-apps/api/webview";
import { Event } from "@tauri-apps/api/event";
import OpenProject from "./components/welcome";
import "./App.css";
import HexProjectWorkspace from "./components/hex-workspace/hex";
import JavaProjectWorkspace from "./components/java-workspace/java"; 


function App() {
  
  const [projectId, setProjectId] = useState<string | null>(null);
  const [projectType, setProjectType] = useState<string | null>(null);
  
  const handleOpenFile = useCallback( async (path: String) => {
    console.log("Opening project at path:", path);
    try {
      const project_id = await invoke<string>("create_project", { path });
      const project_type = await invoke<string>("project_get_type", { projectId: project_id});

      setProjectId(project_id);
      setProjectType(project_type);
    }
    catch (error) {
      console.error("Failed to open project:", error);
      return;
    }
  }, []);

  useEffect(() => {
  
    const setupDragDrop = async () => {
      let webview = getCurrentWebview();
      const handler = (event: Event<DragDropEvent>) => {
        
        const {type } = event.payload as DragDropEvent;
        switch (type) {
          case "enter":
            console.log("Drag enter event detected");
            break;
          case "over": 
            console.log("Drag over event detected");
            break;
          case "leave":
            console.log("Drag leave event detected");
            break;
          case "drop":
            const dropPayload = event.payload as DragDropEvent & { paths?: string[] };
            const paths = dropPayload.paths;
            console.log("Drop event detected:", paths);
            if (paths && paths.length > 0) {
              console.log("Files dropped:", paths);
              handleOpenFile(paths[0]);
            }
            break;
          }
        
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
      onDragOver={e => e.preventDefault()} >
      {!projectType && <OpenProject onOpen={handleOpenFile} />}
      {projectType === "Hex" && <HexProjectWorkspace projectId={projectId} />}
      {projectType === "Java" && <JavaProjectWorkspace projectId={projectId} />}
      {/* 你可以根据需要添加更多类型 */}
    </div>
  );
}

export default App;

