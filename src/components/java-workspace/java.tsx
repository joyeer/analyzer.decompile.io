import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";

interface JavaProjectWorkspaceProps {
  projectId: string | null;
}

export default function JavaProjectWorkspace({ projectId }: JavaProjectWorkspaceProps) {
  const [classFiles, setClassFiles] = useState<string[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [fileContent, setFileContent] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [expandedDirs, setExpandedDirs] = useState<Set<string>>(new Set());

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
      const content = await invoke<string>("java_project_read_file_content", { 
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

  // 切换目录展开/收起状态
  const toggleDirectory = (dirPath: string) => {
    setExpandedDirs(prev => {
      const newSet = new Set(prev);
      if (newSet.has(dirPath)) {
        newSet.delete(dirPath);
      } else {
        newSet.add(dirPath);
      }
      return newSet;
    });
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
      const isExpanded = expandedDirs.has(fullPath);
      
      if (isFile) {
        return (
          <div
            key={fullPath}
            className={`py-1 cursor-pointer hover:bg-gray-100 text-sm font-mono ${
              selectedFile === fullPath ? 'bg-blue-100' : ''
            }`}
            style={{ paddingLeft: `${depth * 16 + 16}px` }}
            onClick={() => handleFileClick(fullPath)}
          >
            📄 {key}
          </div>
        );
      } else {
        return (
          <div key={fullPath}>
            <div
              className="py-1 cursor-pointer hover:bg-gray-100 text-sm flex items-center"
              style={{ paddingLeft: `${depth * 16 + 8}px` }}
              onClick={() => toggleDirectory(fullPath)}
            >
              <span className="mr-1">
                {isExpanded ? '📂' : '📁'}
              </span>
              {key}
            </div>
            {isExpanded && (
              <div>
                {renderTree(tree[key], fullPath, depth + 1)}
              </div>
            )}
          </div>
        );
      }
    });
  };

  const directoryTree = buildDirectoryTree(classFiles);

  return (
    <div className="w-full h-screen bg-white">
      <PanelGroup direction="horizontal">
        {/* 左侧面板：目录结构 */}
        <Panel 
          defaultSize={25} 
          minSize={15} 
          maxSize={50}
          className="border-r border-gray-300"
        >
          <div className="h-full bg-white overflow-y-auto p-2">
            <div className="space-y-1">
              {renderTree(directoryTree)}
            </div>
          </div>
        </Panel>

        {/* 分割线 */}
        <PanelResizeHandle className="w-1.5 bg-gray-200 hover:bg-gray-300 transition-colors duration-200 cursor-col-resize flex items-center justify-center group">
          <div className="w-0.5 h-8 bg-gray-400 group-hover:bg-gray-500 transition-colors duration-200 rounded-full"></div>
        </PanelResizeHandle>

        {/* 右侧面板：文件内容 */}
        <Panel defaultSize={75} minSize={50}>
          <div className="h-full flex flex-col bg-white">
            <div className="p-3 border-b border-gray-300 bg-gray-50 flex-shrink-0">
              <h3 className="text-md font-semibold text-gray-700">
                {selectedFile ? selectedFile : "选择一个文件查看内容"}
              </h3>
            </div>
            <div className="flex-1 overflow-y-auto p-4">
              {loading ? (
                <div className="flex items-center justify-center h-32">
                  <div className="text-center text-gray-500">
                    <div className="inline-block animate-spin rounded-full h-6 w-6 border-b-2 border-blue-500 mb-2"></div>
                    <div>加载中...</div>
                  </div>
                </div>
              ) : selectedFile ? (
                <pre className="text-xs font-mono bg-gray-100 p-4 rounded overflow-x-auto whitespace-pre-wrap">
                  {fileContent}
                </pre>
              ) : (
                <div className="flex items-center justify-center h-full">
                  <div className="text-center text-gray-400">
                    <div className="text-4xl mb-4">📁</div>
                    <div className="text-lg">点击左侧文件查看内容</div>
                  </div>
                </div>
              )}
            </div>
          </div>
        </Panel>
      </PanelGroup>
    </div>
  );
}