import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Panel, PanelGroup, PanelResizeHandle } from "react-resizable-panels";
// 导入 Lucide 图标
import { 
  File,
  Smartphone,
  Archive,
  Code,
  Settings,
  Loader2,
  Database,
  Grid,
  Layers
} from 'lucide-react';

interface AndroidProjectWorkspaceProps {
  projectId: string | null;
}

export default function AndroidProjectWorkspace({ projectId }: AndroidProjectWorkspaceProps) {
  const [analysisResult, setAnalysisResult] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [apkPath, setApkPath] = useState<string | null>(null);

  useEffect(() => {
    if (!projectId) return;
    
    // 获取项目路径
    invoke<string>("project_get_path", { projectId })
      .then(path => {
        setApkPath(path);
        // 自动开始分析
        analyzeApk(path);
      })
      .catch(err => {
        console.error("Failed to get project path:", err);
        setError("Failed to load project");
      });
  }, [projectId]);

  const analyzeApk = async (path: string) => {
    if (!path) return;
    
    setLoading(true);
    setError(null);
    setAnalysisResult("");
    
    try {
      // 调用 Rust 端的 Android APK 分析命令
      const result = await invoke<string>("android_analyze_apk", { 
        apkPath: path 
      });
      setAnalysisResult(result);
    } catch (err) {
      console.error("Failed to analyze APK:", err);
      setError(`Analysis failed: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleReanalyze = () => {
    if (apkPath) {
      analyzeApk(apkPath);
    }
  };

  const getFileIcon = (fileName: string) => {
    const ext = fileName.split('.').pop()?.toLowerCase();
    switch (ext) {
      case 'apk':
        return <Archive className="w-4 h-4 text-green-600" />;
      case 'dex':
        return <Database className="w-4 h-4 text-blue-600" />;
      case 'xml':
        return <Code className="w-4 h-4 text-orange-600" />;
      case 'arsc':
        return <Grid className="w-4 h-4 text-purple-600" />;
      default:
        return <File className="w-4 h-4 text-gray-600" />;
    }
  };

  return (
    <div className="w-full h-screen bg-gray-50">
      <div className="border-b border-gray-200 bg-white px-4 py-3">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <Smartphone className="w-5 h-5 text-green-600" />
            <h1 className="text-lg font-semibold text-gray-900">Android Project Analyzer</h1>
          </div>
          <div className="flex items-center space-x-2">
            <button
              onClick={handleReanalyze}
              disabled={loading || !apkPath}
              className="px-3 py-1 bg-blue-500 text-white rounded text-sm hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed flex items-center space-x-1"
            >
              {loading ? <Loader2 className="w-4 h-4 animate-spin" /> : <Settings className="w-4 h-4" />}
              <span>{loading ? "Analyzing..." : "Re-analyze"}</span>
            </button>
          </div>
        </div>
      </div>

      <PanelGroup direction="horizontal" className="h-full">
        {/* 左侧面板 - 项目信息 */}
        <Panel defaultSize={30} minSize={20} maxSize={50}>
          <div className="h-full bg-white border-r border-gray-200 overflow-y-auto">
            <div className="p-4">
              <div className="flex items-center space-x-2 mb-4">
                <Layers className="w-5 h-5 text-blue-600" />
                <h2 className="text-lg font-semibold text-gray-900">Project Info</h2>
              </div>
              
              {apkPath && (
                <div className="mb-4 p-3 bg-gray-50 rounded-lg">
                  <div className="flex items-center space-x-2 mb-2">
                    {getFileIcon(apkPath)}
                    <span className="text-sm font-medium text-gray-700">APK Path:</span>
                  </div>
                  <p className="text-sm text-gray-600 break-all">{apkPath}</p>
                </div>
              )}

              {error && (
                <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-lg">
                  <div className="flex items-center space-x-2 mb-2">
                    <div className="w-4 h-4 bg-red-500 rounded-full"></div>
                    <span className="text-sm font-medium text-red-700">Error</span>
                  </div>
                  <p className="text-sm text-red-600">{error}</p>
                </div>
              )}

              {loading && (
                <div className="mb-4 p-3 bg-blue-50 border border-blue-200 rounded-lg">
                  <div className="flex items-center space-x-2">
                    <Loader2 className="w-4 h-4 animate-spin text-blue-600" />
                    <span className="text-sm font-medium text-blue-700">Analyzing APK...</span>
                  </div>
                  <p className="text-sm text-blue-600 mt-2">
                    This may take a moment depending on the APK size.
                  </p>
                </div>
              )}

              <div className="space-y-2">
                <div className="flex items-center space-x-2">
                  <Database className="w-4 h-4 text-blue-600" />
                  <span className="text-sm font-medium text-gray-700">Analysis Components:</span>
                </div>
                <ul className="text-sm text-gray-600 space-y-1 ml-6">
                  <li>• APK Structure</li>
                  <li>• DEX Files</li>
                  <li>• AndroidManifest.xml</li>
                  <li>• Resources (ARSC)</li>
                  <li>• Class Definitions</li>
                  <li>• Method Signatures</li>
                </ul>
              </div>
            </div>
          </div>
        </Panel>

        <PanelResizeHandle className="w-2 bg-gray-200 hover:bg-gray-300 transition-colors" />

        {/* 右侧面板 - 分析结果 */}
        <Panel defaultSize={70} minSize={50}>
          <div className="h-full bg-white overflow-y-auto">
            <div className="p-4">
              <div className="flex items-center space-x-2 mb-4">
                <Code className="w-5 h-5 text-green-600" />
                <h2 className="text-lg font-semibold text-gray-900">Analysis Results</h2>
              </div>
              
              <div className="bg-gray-50 rounded-lg p-4 border">
                {loading ? (
                  <div className="flex items-center justify-center py-8">
                    <Loader2 className="w-6 h-6 animate-spin text-blue-600" />
                    <span className="ml-2 text-gray-600">Analyzing APK...</span>
                  </div>
                ) : analysisResult ? (
                  <pre className="text-sm text-gray-800 whitespace-pre-wrap font-mono overflow-x-auto">
                    {analysisResult}
                  </pre>
                ) : error ? (
                  <div className="text-center py-8 text-red-600">
                    <p>Analysis failed. Please check the error above and try again.</p>
                  </div>
                ) : (
                  <div className="text-center py-8 text-gray-500">
                    <Smartphone className="w-12 h-12 mx-auto mb-4 text-gray-400" />
                    <p>No analysis results yet. Click "Re-analyze" to start.</p>
                  </div>
                )}
              </div>
            </div>
          </div>
        </Panel>
      </PanelGroup>
    </div>
  );
}
