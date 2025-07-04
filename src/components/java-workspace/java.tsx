import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
// 导入 Lucide 图标
import { 
  Folder, 
  FolderOpen, 
  File,
  FileText,
  Coffee,
  Package,
  Archive,
  Code,
  Braces,
  Settings,
  Loader2
} from 'lucide-react';

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

  // 根据文件扩展名返回对应的图标
  const getFileIcon = (fileName: string) => {
    const extension = fileName.toLowerCase().split('.').pop();
    
    switch (extension) {
      case 'java':
        return <Coffee className="text-orange-500" size={16} />;
      case 'class':
        return <Code className="text-blue-500" size={16} />;
      case 'jar':
        return <Archive className="text-green-600" size={16} />;
      case 'json':
        return <Braces className="text-yellow-500" size={16} />;
      case 'xml':
        return <Code className="text-orange-400" size={16} />;
      case 'properties':
        return <Settings className="text-gray-500" size={16} />;
      case 'md':
        return <FileText className="text-blue-400" size={16} />;
      case 'txt':
        return <FileText className="text-gray-600" size={16} />;
      default:
        return <File className="text-gray-500" size={16} />;
    }
  };

  // 获取目录图标
  const getFolderIcon = (isExpanded: boolean) => {
    return isExpanded ? 
      <FolderOpen className="text-blue-500" size={16} /> : 
      <Folder className="text-blue-500" size={16} />;
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
            className={`py-1 px-1 cursor-pointer hover:bg-gray-100 text-sm font-mono flex items-center rounded transition-colors ${
              selectedFile === fullPath ? 'bg-blue-100 text-blue-800' : 'text-gray-700'
            }`}
            style={{ paddingLeft: `${depth * 16 + 16}px` }}
            onClick={() => handleFileClick(fullPath)}
          >
            <span className="mr-2 flex-shrink-0">
              {getFileIcon(key)}
            </span>
            <span className="truncate">{key}</span>
          </div>
        );
      } else {
        return (
          <div key={fullPath}>
            <div
              className="py-1 px-1 cursor-pointer hover:bg-gray-100 text-sm flex items-center rounded transition-colors text-gray-700"
              style={{ paddingLeft: `${depth * 16 + 8}px` }}
              onClick={() => toggleDirectory(fullPath)}
            >
              <span className="mr-2 flex-shrink-0">
                {getFolderIcon(isExpanded)}
              </span>
              <span className="truncate">{key}</span>
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
            {/* 添加标题栏 */}
            <div className="pb-2 mb-2 border-b border-gray-200">
              <h4 className="text-xs font-semibold text-gray-600 uppercase tracking-wide flex items-center">
                <Package className="mr-1" size={14} />
                Explorer
              </h4>
            </div>
            <div className="space-y-0.5">
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
              <h3 className="text-md font-semibold text-gray-700 flex items-center">
                {selectedFile ? (
                  <>
                    <span className="mr-2">
                      {getFileIcon(selectedFile.split('/').pop() || '')}
                    </span>
                    {selectedFile}
                  </>
                ) : (
                  <>
                    <File className="mr-2" size={16} />
                    选择一个文件查看内容
                  </>
                )}
              </h3>
            </div>
            <div className="flex-1 overflow-y-auto p-4">
              {loading ? (
                <div className="flex items-center justify-center h-32">
                  <div className="text-center text-gray-500">
                    <Loader2 className="animate-spin h-6 w-6 mx-auto mb-2" />
                    <div>加载中...</div>
                  </div>
                </div>
              ) : selectedFile ? (
                <pre className="text-xs font-mono rounded overflow-x-auto whitespace-pre-wrap">
                  {fileContent}
                </pre>
              ) : (
                <div className="flex items-center justify-center h-full">
                  <div className="text-center text-gray-400">
                    <Folder size={48} className="mx-auto mb-4" />
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