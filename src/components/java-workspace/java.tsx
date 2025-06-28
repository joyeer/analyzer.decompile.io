import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface JavaProjectWorkspaceProps {
  projectId: string | null;
}

export default function JavaProjectWorkspace({ projectId }: JavaProjectWorkspaceProps) {
  const [classFiles, setClassFiles] = useState<string[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [fileContent, setFileContent] = useState<string>("");
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!projectId) return;
    
    // 获取 Java 项目的 class 文件列表
    invoke<string[]>("java_project_list_files", { projectId })
      .then(setClassFiles)
      .catch(console.error);
  }, [projectId]);

  const handleFileClick = async (fileName: string) => {
    if (!projectId) return;
    
    setSelectedFile(fileName);
    setLoading(true);
    setFileContent("");
    
    try {
      // 调用 Rust 端读取文件内容的命令
      const content = await invoke<string>("java_read_file_content", { 
        projectId, 
        fileName 
      });
      setFileContent(content);
    } catch (error) {
      console.error("Failed to read file:", error);
      setFileContent("读取文件失败");
    } finally {
      setLoading(false);
    }
  };

  // 构建目录树结构
  const buildDirectoryTree = (files: string[]) => {
    const tree: { [key: string]: any } = {};
    
    files.forEach(file => {
      const parts = file.split('/');
      let current = tree;
      
      parts.forEach((part, index) => {
        if (!current[part]) {
          current[part] = index === parts.length - 1 ? null : {};
        }
        if (current[part] !== null) {
          current = current[part];
        }
      });
    });
    
    return tree;
  };

  const renderTree = (tree: any, path = "", depth = 0): React.ReactNode => {
    return Object.keys(tree).map(key => {
      const fullPath = path ? `${path}/${key}` : key;
      const isFile = tree[key] === null;
      const indent = depth * 16; // 每层缩进 16px
      
      if (isFile) {
        return (
          <div
            key={fullPath}
            className={`py-1 cursor-pointer hover:bg-blue-100 text-sm font-mono ${
              selectedFile === fullPath ? 'bg-blue-200' : ''
            }`}
            style={{ paddingLeft: `${indent + 16}px` }}
            onClick={() => handleFileClick(fullPath)}
          >
            📄 {key}
          </div>
        );
      } else {
        return (
          <div key={fullPath}>
            <details open>
              <summary 
                className="py-1 cursor-pointer hover:bg-gray-100 text-sm font-semibold list-none"
                style={{ paddingLeft: `${indent + 8}px` }}
              >
                📁 {key}
              </summary>
            </details>
            {renderTree(tree[key], fullPath, depth + 1)}
          </div>
        );
      }
    });
  };

  const directoryTree = buildDirectoryTree(classFiles);

  return (
    <div className="w-full h-screen bg-white flex">
      {/* 左侧：目录结构 */}
      <div className="w-1/3 border-r border-gray-300 overflow-y-auto p-2">
        <div className="space-y-1">
          {renderTree(directoryTree)}
        </div>
      </div>

      {/* 右侧：文件内容 */}
      <div className="w-2/3 flex flex-col">
        <div className="p-2 border-b border-gray-300 bg-gray-50">
          <h3 className="text-md font-semibold text-gray-700">
            {selectedFile ? selectedFile : "选择一个文件查看内容"}
          </h3>
        </div>
        <div className="flex-1 overflow-y-auto p-4">
          {loading ? (
            <div className="text-center text-gray-500">加载中...</div>
          ) : selectedFile ? (
            <pre className="text-xs font-mono bg-gray-100 p-4 rounded overflow-x-auto">
              {fileContent}
            </pre>
          ) : (
            <div className="text-center text-gray-400 mt-20">
              点击左侧文件查看内容
            </div>
          )}
        </div>
      </div>
    </div>
  );
}