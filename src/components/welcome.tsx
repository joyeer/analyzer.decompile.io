
import { useState } from "react";
import { open } from "@tauri-apps/plugin-dialog";

export default function OpenProject({ onOpen }: { onOpen?: (path: String) => void }) {
  const [selectedPath, setSelectedPath] = useState("");

  const selectFileOrFolder = async () => {
    try {
      // Open the file/folder picker dialog
      const path = await open({
        directory: false, // Set to `true` if you want to select a folder
        multiple: false, // Set to `true` if you want to select multiple files
      });

      // Set the selected path
      if (path) {
        setSelectedPath(path);
        if (onOpen) {
          onOpen(path);
        }
      }
    } catch (error) {
      console.error("Error selecting file/folder:", error);
    }
  };

  return (
    <div>
      <button onClick={selectFileOrFolder}>Select File/Folder</button>
      {selectedPath && <p>Selected Path: {selectedPath}</p>}
    </div>
  );
}