import React, { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface HexProjectWorkspaceProps {
  projectId: string | null;
}

const PAGE_SIZE = 256;

export default function HexProjectWorkspace({ projectId }: HexProjectWorkspaceProps) {
  const [page, setPage] = useState(0);
  const [hexData, setHexData] = useState<Uint8Array[]>([]);
  const [totalPages, setTotalPages] = useState<number>(1);
  const containerRef = useRef<HTMLDivElement>(null);
  const loadingRef = useRef(false);

  // 加载总页数和第一页
  useEffect(() => {
    if (!projectId) return;
    invoke<number>("hex_project_total_pages", { project_id: projectId, page_size: PAGE_SIZE })
      .then(setTotalPages)
      .catch(() => setTotalPages(1));
    setHexData([]);
    setPage(0);
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
    invoke<Uint8Array>("hex_project_read_page", {
      project_id: projectId,
      page: pageNum,
      page_size: PAGE_SIZE,
    }).then((data) => {
      setHexData((prev) => (replace ? [data] : [...prev, data]));
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
            <span className="w-80">{hex}</span>
            <span className="ml-4">{ascii}</span>
          </div>
        );
      }
      offset += data.length;
    });
    return <div className="bg-black text-green-400 p-4 rounded">{lines}</div>;
  };

  return (
    <div className="w-full max-w-3xl mx-auto mt-8">
      <h2 className="text-lg font-bold mb-4">Hex 文件浏览</h2>
      <div
        ref={containerRef}
        className="overflow-y-auto"
        style={{ maxHeight: 480, minHeight: 320 }}
      >
        {renderHex()}
        {page < totalPages - 1 && (
          <div className="text-center text-gray-400 py-2">滚动加载更多...</div>
        )}
      </div>
    </div>
  );
}