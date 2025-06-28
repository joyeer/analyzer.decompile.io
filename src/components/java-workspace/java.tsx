import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface JavaProjectWorkspaceProps {
  projectId: string | null;
}

export default function JavaProjectWorkspace({ projectId }: JavaProjectWorkspaceProps) {
  const [classFiles, setClassFiles] = useState<string[]>([]);

  useEffect(() => {
    if (!projectId) return;
    
    // 获取 Java 项目的 class 文件列表
    invoke<string[]>("java_project_list_files", { projectId })
      .then(setClassFiles)
      .catch(console.error);
  }, [projectId]);

  return (
    <div className="w-full h-screen bg-white p-2">
      <div className="h-full overflow-y-auto">
        <h2 className="text-lg font-bold mb-4">Java 项目</h2>
        <div className="space-y-2">
          {classFiles.map((file, index) => (
            <div key={index} className="p-2 bg-gray-100 rounded text-sm font-mono">
              {file}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}