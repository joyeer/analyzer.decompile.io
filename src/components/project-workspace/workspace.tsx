import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
import { 
  Folder, 
  FolderOpen, 
  File,
  FileText,
  Coffee,
  Smartphone,
  Archive,
  Code,
  Braces,
  Settings,
  Loader2,
  Database,
  Grid,
  Package,
  Play,
} from 'lucide-react';

interface ProjectWorkspaceProps {
  projectId: string | null;
  projectType: string | null;
}

export default function ProjectWorkspace({ projectId, projectType }: ProjectWorkspaceProps) {
  const [classFiles, setClassFiles] = useState<string[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [fileContent, setFileContent] = useState<string>("");
  const [analysisResult, setAnalysisResult] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [analyzing, setAnalyzing] = useState(false);
  const [expandedDirs, setExpandedDirs] = useState<Set<string>>(new Set());
  const [projectPath, setProjectPath] = useState<string>("");

  useEffect(() => {
    if (!projectId || !projectType) return;
    
    loadProjectFiles();
  }, [projectId, projectType]);

  const loadProjectFiles = async () => {
    if (!projectId || !projectType) return;
    
    try {
      // 获取项目路径
      const path = await invoke<string>("project_get_path", { projectId });
      setProjectPath(path);
      
      // 根据项目类型加载文件列表
      if (projectType === "Java") {
        const files = await invoke<string[]>("java_project_list_files", { projectId });
        setClassFiles(files);
      } else if (projectType === "Android") {
        // Android项目自动开始分析
        analyzeAndroidProject(path);
      }
    } catch (error) {
      console.error("Failed to load project files:", error);
    }
  };

  const handleFileClick = async (fileName: string) => {
    if (!projectId || projectType !== "Java") return;
    
    setSelectedFile(fileName);
    setLoading(true);
    setFileContent("");
    
    try {
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

  const analyzeAndroidProject = async (apkPath?: string) => {
    const path = apkPath || projectPath;
    if (!path) return;
    
    setAnalyzing(true);
    setAnalysisResult("");
    
    try {
      const result = await invoke<string>("android_analyze_apk", { 
        apkPath: path 
      });
      setAnalysisResult(result);
    } catch (error) {
      console.error("Failed to analyze APK:", error);
      setAnalysisResult(`分析失败: ${error}`);
    } finally {
      setAnalyzing(false);
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
      case 'apk':
        return <Archive className="text-green-600" size={16} />;
      case 'dex':
        return <Database className="text-blue-500" size={16} />;
      case 'json':
        return <Braces className="text-yellow-500" size={16} />;
      case 'xml':
        return <Code className="text-orange-400" size={16} />;
      case 'arsc':
        return <Grid className="text-purple-500" size={16} />;
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

  // 获取项目类型图标
  const getProjectTypeIcon = () => {
    switch (projectType) {
      case 'Java':
        return <Coffee className="text-orange-500" size={16} />;
      case 'Android':
        return <Smartphone className="text-green-500" size={16} />;
      default:
        return <Package className="text-gray-500" size={16} />;
    }
  };

  const renderTree = (tree: any, path = "", depth = 0) => {
    return Object.keys(tree).map(key => {
      const fullPath = path ? `${path}/${key}` : key;
      const isFile = tree[key] === null;
      const isExpanded = expandedDirs.has(fullPath);
      
      if (isFile) {
        return (
          <div
            key={fullPath}
            className={`py-1 px-1 cursor-pointer hover:bg-gray-100 text-sm flex items-center rounded transition-colors ${
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
        {/* 左侧面板：项目结构 */}
        <Panel 
          defaultSize={25} 
          minSize={15} 
          maxSize={50}
          className="border-r border-gray-300"
        >
          <div className="h-full bg-white overflow-y-auto p-2 project-explorer">
            {/* 项目标题栏 */}
            <div className="pb-2 mb-2 border-b border-gray-200">
              <h4 className="text-xs font-semibold text-gray-600 uppercase tracking-wide flex items-center">
                {getProjectTypeIcon()}
                <span className="ml-1">{projectType} Project</span>
              </h4>
              {projectPath && (
                <div className="text-xs text-gray-500 mt-1 truncate" title={projectPath}>
                  {projectPath.split('/').pop()}
                </div>
              )}
            </div>

            {/* 项目文件结构 */}
            <div className="space-y-0.5">
              {projectType === "Java" && classFiles.length > 0 ? (
                renderTree(directoryTree)
              ) : projectType === "Android" ? (
                <div className="space-y-2">
                  <div className="text-sm text-gray-600 p-2 bg-gray-50 rounded">
                    <div className="font-medium mb-2">APK 组件:</div>
                    <div className="space-y-1 text-xs">
                      <div className="flex items-center space-x-2">
                        <Database size={12} className="text-blue-500" />
                        <span>DEX 文件</span>
                      </div>
                      <div className="flex items-center space-x-2">
                        <Code size={12} className="text-orange-500" />
                        <span>AndroidManifest.xml</span>
                      </div>
                      <div className="flex items-center space-x-2">
                        <Grid size={12} className="text-purple-500" />
                        <span>资源文件 (ARSC)</span>
                      </div>
                      <div className="flex items-center space-x-2">
                        <Archive size={12} className="text-green-500" />
                        <span>类定义和方法</span>
                      </div>
                    </div>
                  </div>
                  {projectPath && (
                    <button
                      onClick={() => analyzeAndroidProject()}
                      disabled={analyzing}
                      className="w-full px-3 py-2 bg-blue-500 text-white rounded text-sm hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed flex items-center justify-center space-x-2"
                    >
                      {analyzing ? (
                        <>
                          <Loader2 size={14} className="animate-spin" />
                          <span>分析中...</span>
                        </>
                      ) : (
                        <>
                          <Play size={14} />
                          <span>重新分析</span>
                        </>
                      )}
                    </button>
                  )}
                </div>
              ) : (
                <div className="text-sm text-gray-500 text-center py-8">
                  {projectType === "Java" ? "无文件" : "加载中..."}
                </div>
              )}
            </div>
          </div>
        </Panel>

        {/* 分割线 */}
        <PanelResizeHandle className="w-1.5 bg-gray-200 hover:bg-gray-300 transition-colors duration-200 cursor-col-resize flex items-center justify-center group">
          <div className="w-0.5 h-8 bg-gray-400 group-hover:bg-gray-500 transition-colors duration-200 rounded-full"></div>
        </PanelResizeHandle>

        {/* 右侧面板：内容显示 */}
        <Panel defaultSize={75} minSize={50}>
          <div className="h-full flex flex-col bg-white">
            {/* 顶部标题栏 */}
            <div className="p-3 border-b border-gray-300 bg-gray-50 flex-shrink-0">
              <h3 className="text-md font-semibold text-gray-700 flex items-center">
                {projectType === "Java" && selectedFile ? (
                  <>
                    <span className="mr-2">
                      {getFileIcon(selectedFile.split('/').pop() || '')}
                    </span>
                    {selectedFile}
                  </>
                ) : projectType === "Android" ? (
                  <>
                    <Smartphone className="mr-2" size={16} />
                    APK 分析结果
                  </>
                ) : (
                  <>
                    <File className="mr-2" size={16} />
                    {projectType === "Java" ? "选择一个文件查看内容" : "内容"}
                  </>
                )}
              </h3>
            </div>

            {/* 内容区域 */}
            <div className="flex-1 overflow-y-auto p-4">
              {loading || analyzing ? (
                <div className="flex items-center justify-center h-32">
                  <div className="text-center text-gray-500">
                    <Loader2 className="animate-spin h-6 w-6 mx-auto mb-2" />
                    <div>{loading ? "加载中..." : "分析中..."}</div>
                  </div>
                </div>
              ) : (projectType === "Java" && selectedFile && fileContent) ? (
                <pre className="text-xs font-mono rounded overflow-x-auto whitespace-pre-wrap">
                  {fileContent}
                </pre>
              ) : (projectType === "Android" && analysisResult) ? (
                <pre className="text-xs font-mono rounded overflow-x-auto whitespace-pre-wrap bg-gray-50 p-4">
                  {analysisResult}
                </pre>
              ) : (
                <div className="flex items-center justify-center h-full">
                  <div className="text-center text-gray-400">
                    {projectType === "Java" ? (
                      <>
                        <Folder size={48} className="mx-auto mb-4" />
                        <div className="text-lg">点击左侧文件查看内容</div>
                      </>
                    ) : (
                      <>
                        <Smartphone size={48} className="mx-auto mb-4" />
                        <div className="text-lg">点击左侧 "重新分析" 开始 APK 分析</div>
                      </>
                    )}
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
