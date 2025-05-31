import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [projectId, setProjectId] = useState<string>("");
  const handleDrop = async (event: React.DragEvent<HTMLDivElement>) => {
    event.preventDefault();
    const file = event.dataTransfer.files[0];
    if (file) {
      // Tauri: 获取文件路径（仅在 Tauri 环境下有效）
      // @ts-ignore
      const path = (file as any).path;
      if (!path) {
        console.error('无法获取文件路径，确保在 Tauri 环境下运行。');
        return;
      }
      // 你可以自定义 name/description/version
      const projectId = await invoke<string>('create_project', {
        name: file.name,
        path,
        description: '用户拖拽创建',
        version: '1.0.0'
      });
      // 保存 projectId 或做后续处理
      console.log('新建 Project ID:', projectId);
    }
  }
  
  return (
    <main className="container">
      <div style={{
        height: '100%',
        width: '100%',
      }} 
      onDrop={handleDrop}/>
    </main>
  );
}

export default App;

