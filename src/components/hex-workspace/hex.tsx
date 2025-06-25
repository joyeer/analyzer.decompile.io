import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface HexProjectWorkspaceProps {
  projectId: string | null;
}

const PAGE_SIZE = 1024 * 8;

export default function HexProjectWorkspace({ projectId }: HexProjectWorkspaceProps) {
  const [page, setPage] = useState(0);
  const [hexData, setHexData] = useState<Uint8Array[]>([]);
  const [totalPages, setTotalPages] = useState<number>(1);
  const containerRef = useRef<HTMLDivElement>(null);
  const loadingRef = useRef(false);

  // 加载总页数和第一页
  useEffect(() => {
    if (!projectId) return;
    // 获取文件尺寸
    invoke<number>("hex_project_get_file_size", { projectId: projectId })
      .then(size => console.log("文件大小:", size));
      
    // 获取总页数
    invoke<number>("hex_project_get_total_pages", { 
      projectId: projectId, 
      pageSize: PAGE_SIZE 
  }).then(setTotalPages);
  
  // 加载第一页
  loadPage(0, true);
  }, [projectId]);

  // 滚动到底部时加载下一页
  useEffect(() => {
    const onScroll = () => {
      if (!containerRef.current || loadingRef.current) return;
      const { scrollTop, scrollHeight, clientHeight } = containerRef.current;
      if (scrollTop + clientHeight >= scrollHeight - 10) {
        if (page < totalPages - 1) {
          loadPage(page + 1);
        }
      }
    };
    const el = containerRef.current;
    if (el) el.addEventListener("scroll", onScroll);
    return () => {
      if (el) el.removeEventListener("scroll", onScroll);
    };
  }, [page, totalPages, projectId]);

  // 加载指定页
  const loadPage = (pageNum: number, replace = false) => {
  if (!projectId || loadingRef.current) return;
  loadingRef.current = true;
  
  invoke<number[]>("hex_project_read_page", {
    projectId: projectId,
    page: pageNum,
    pageSize: PAGE_SIZE,
  }).then((data) => {
    const uint8Data = new Uint8Array(data);
    setHexData((prev) => (replace ? [uint8Data] : [...prev, uint8Data]));
    setPage(pageNum);
    loadingRef.current = false;
  });
};

  // 渲染 hex 视图
  const renderHex = () => {
    if (hexData.length === 0) return <div className="text-gray-400">暂无数据</div>;
    let lines: React.ReactNode[] = [];
    let offset = 0;
    hexData.forEach((data, pageIdx) => {
      for (let i = 0; i < data.length; i += 16) {
        const chunk = data.slice(i, i + 16);
        const hex = Array.from(chunk)
          .map((b) => b.toString(16).padStart(2, "0"))
          .join(" ");
        const ascii = Array.from(chunk)
          .map((b) => (b >= 32 && b <= 126 ? String.fromCharCode(b) : "."))
          .join("");
        lines.push(
          <div key={offset + i} className="font-mono text-xs flex">
            <span className="w-24 text-gray-400">{(pageIdx * PAGE_SIZE + i).toString(16).padStart(8, "0")}</span>
            <span className="w-90">{hex}</span>
            <span className="ml-4">{ascii}</span>
          </div>
        );
      }
      offset += data.length;
    });
    return <div className="bg-white">{lines}</div>;
  };

  return (
    <div className="w-full h-screen bg-white">
      <div
        ref={containerRef}
        className="overflow-y-auto h-full"
      >
        {renderHex()}
      </div>
    </div>
  );
}