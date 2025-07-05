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
      // Get project path
      const path = await invoke<string>("project_get_path", { projectId });
      setProjectPath(path);
      
      // Load file list based on project type
      if (projectType === "Java") {
        const files = await invoke<string[]>("java_project_list_files", { projectId });
        setClassFiles(files);
      } else if (projectType === "Android") {
        // Load APK file structure
        const files = await invoke<string[]>("android_project_list_files", { projectId });
        setClassFiles(files);
        // Also start analysis
        analyzeAndroidProject(path);
      }
    } catch (error) {
      console.error("Failed to load project files:", error);
    }
  };

  const handleFileClick = async (fileName: string) => {
    if (!projectId) return;
    
    setSelectedFile(fileName);
    setLoading(true);
    setFileContent("");
    
    try {
      let content: string;
      if (projectType === "Java") {
        content = await invoke<string>("java_project_read_file_content", { 
          projectId, 
          fileName 
        });
      } else if (projectType === "Android") {
        content = await invoke<string>("android_project_read_file_content", { 
          projectId, 
          fileName 
        });
      } else {
        content = "Unsupported project type";
      }
      setFileContent(content);
    } catch (error) {
      console.error("Failed to read file:", error);
      setFileContent("Failed to read file");
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
      setAnalysisResult(`Analysis failed: ${error}`);
    } finally {
      setAnalyzing(false);
    }
  };

  // Toggle directory expand/collapse state
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

  // Build directory tree structure
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

  // Return corresponding icon based on file extension
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

  // Get directory icon
  const getFolderIcon = (isExpanded: boolean) => {
    return isExpanded ? 
      <FolderOpen className="text-blue-500" size={16} /> : 
      <Folder className="text-blue-500" size={16} />;
  };

  // Get project type icon
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
    <div className="w-full h-screen bg-white flex flex-col">
      <PanelGroup direction="horizontal" className="flex-1">
        {/* Left panel: Project structure */}
        <Panel 
          defaultSize={25} 
          minSize={15} 
          maxSize={50}
          className="border-r border-gray-300"
        >
          <div className="h-full bg-white overflow-y-auto p-2 project-explorer">
            {/* Project title bar */}
            <div className="pb-2 mb-2 border-b border-gray-200">
              <h4 className="text-sm font-semibold text-gray-700 flex items-center">
                {getProjectTypeIcon()}
                <span className="ml-2 truncate" title={projectPath}>
                  {projectPath ? projectPath.split('/').pop() : 'Project'}
                </span>
              </h4>
            </div>

            {/* Project file structure */}
            <div className="space-y-0.5">
              {(projectType === "Java" || projectType === "Android") && classFiles.length > 0 ? (
                renderTree(directoryTree)
              ) : projectType === "Android" ? (
                <div className="space-y-2">
                  <div className="text-sm text-gray-600 p-2 bg-gray-50 rounded">
                    <div className="font-medium mb-2">APK Components:</div>
                    <div className="space-y-1 text-xs">
                      <div className="flex items-center space-x-2">
                        <Database size={12} className="text-blue-500" />
                        <span>DEX Files</span>
                      </div>
                      <div className="flex items-center space-x-2">
                        <Code size={12} className="text-orange-500" />
                        <span>AndroidManifest.xml</span>
                      </div>
                      <div className="flex items-center space-x-2">
                        <Grid size={12} className="text-purple-500" />
                        <span>Resource Files (ARSC)</span>
                      </div>
                      <div className="flex items-center space-x-2">
                        <Archive size={12} className="text-green-500" />
                        <span>Class Definitions and Methods</span>
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
                          <span>Analyzing...</span>
                        </>
                      ) : (
                        <>
                          <Play size={14} />
                          <span>Re-analyze</span>
                        </>
                      )}
                    </button>
                  )}
                </div>
              ) : (
                <div className="text-sm text-gray-500 text-center py-8">
                  {(projectType === "Java" || projectType === "Android") ? "No files" : "Loading..."}
                </div>
              )}
            </div>
          </div>
        </Panel>

        {/* Divider */}
        <PanelResizeHandle className="w-0.5 bg-gray-200 hover:bg-gray-300 transition-colors duration-200 cursor-col-resize flex items-center justify-center group">
          <div className="w-0.5 h-3 bg-gray-300 group-hover:bg-gray-400 transition-colors duration-200 opacity-40 group-hover:opacity-60"></div>
        </PanelResizeHandle>

        {/* Right panel: Content display */}
        <Panel defaultSize={75} minSize={50}>
          <div className="h-full flex flex-col bg-white">
            {/* Top title bar */}
            <div className="p-3 border-b border-gray-300 bg-gray-50 flex-shrink-0">
              <h3 className="text-md font-semibold text-gray-700 flex items-center">
                {(projectType === "Java" || projectType === "Android") && selectedFile ? (
                  <>
                    <span className="mr-2">
                      {getFileIcon(selectedFile.split('/').pop() || '')}
                    </span>
                    {selectedFile}
                  </>
                ) : projectType === "Android" && !selectedFile ? (
                  <>
                    <Smartphone className="mr-2" size={16} />
                    APK Analysis Results
                  </>
                ) : (
                  <>
                    <File className="mr-2" size={16} />
                    {(projectType === "Java" || projectType === "Android") ? "Select a file to view content" : "Content"}
                  </>
                )}
              </h3>
            </div>

            {/* Content area */}
            <div className="flex-1 overflow-y-auto p-4">
              {loading || analyzing ? (
                <div className="flex items-center justify-center h-32">
                  <div className="text-center text-gray-500">
                    <Loader2 className="animate-spin h-6 w-6 mx-auto mb-2" />
                    <div>{loading ? "Loading..." : "Analyzing..."}</div>
                  </div>
                </div>
              ) : ((projectType === "Java" || projectType === "Android") && selectedFile && fileContent) ? (
                <pre className="text-xs font-mono rounded overflow-x-auto whitespace-pre-wrap">
                  {fileContent}
                </pre>
              ) : (projectType === "Android" && !selectedFile && analysisResult) ? (
                <pre className="text-xs font-mono rounded overflow-x-auto whitespace-pre-wrap bg-gray-50 p-4">
                  {analysisResult}
                </pre>
              ) : (
                <div className="flex items-center justify-center h-full">
                  <div className="text-center text-gray-400">
                    {(projectType === "Java" || projectType === "Android") ? (
                      <>
                        <Folder size={48} className="mx-auto mb-4" />
                        <div className="text-lg">Click on a file in the left panel to view content</div>
                      </>
                    ) : (
                      <>
                        <Smartphone size={48} className="mx-auto mb-4" />
                        <div className="text-lg">Click "Re-analyze" in the left panel to start APK analysis</div>
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
