"use client";
import styles from "./page.module.css";
import { PrimaryButton } from "@fluentui/react";
import { invoke } from "@tauri-apps/api/tauri";
import { ComboBox } from "@fluentui/react";
import { useEffect, useState } from "react";
import { register } from "@tauri-apps/api/globalShortcut";

enum TextDirection {
  Horizontal = "horizontal",
  Vertical = "vertical",
}

const tesseractArguments = {
  [TextDirection.Vertical]: { lang: "jpn_vert", psm: 5 },
  [TextDirection.Horizontal]: { lang: "jpn", psm: 6 },
};

const parseTsvOutput = (output: string) => {
  const lines = output.includes("\r\n")
    ? output.trim().split("\r\n")
    : output.trim().split("\n");
  const headerRow = lines[0];
  const headers = headerRow.split(/\s+/);
  const dataRows = lines.slice(1);
  const parsedRows = dataRows.map((line) => {
    const cells = line.split(/\s+/);
    let parsedRow: Record<string, string | number> = {};
    headers.forEach((header, i) => {
      parsedRow[header] = header === "text" ? cells[i] : Number(cells[i]);
    });
    return parsedRow;
  });
  return parsedRows;
};

const invokeFunction =
  (functionName: string, args = {}) =>
  async () => {
    console.log(`invoking ${functionName}`, args);
    try {
      const result = await invoke<string>(functionName, args);
      console.log(result);
    } catch (e) {
      console.log("error:", e);
    }
  };

const initiateOcr = async () => {
  const invocationFunction = invokeFunction("ipc_create_window");
  invocationFunction();
};

export default function Home() {
  const comboId = "combo-psm";
  const [textDirection, setTextDirection] = useState<TextDirection>(
    TextDirection.Vertical
  );

  useEffect(() => {
    try {
      register("CommandOrControl+Shift+A", initiateOcr);
      console.log("global shortcut CommandOrControl+Shift+A registered");
    } catch (err) {
      if (!(err as any).includes("hotkey already registered")) {
        console.log("failed to register global shortcut: ", err);
      }
    }
  }, []);

  const runOcrClicked = async () => {
    try {
      const output = await invoke<string>(
        "ipc_ocr",
        tesseractArguments[textDirection]
      );
      if (!output) {
        console.log("no output for some reason", output);
        return;
      }
      let parsedRows = parseTsvOutput(output);
      parsedRows = parsedRows.filter(
        (characterData) => (characterData.conf as number) > 1
      );
      console.table(parsedRows);
    } catch (e) {
      console.log("error:", e);
    }
  };

  return (
    <main className={styles.main}>
      <div className={styles.description}>
        <h2>Yomikun v0.1</h2>
      </div>

      <div className={styles.center}>
        <div>
          <label id={comboId}>Text direction</label>
          <ComboBox
            aria-labelledby={comboId}
            placeholder="Select from dropdown..."
            selectedKey={textDirection}
            onChange={(e, option) => setTextDirection(option?.key as any)}
            options={[
              { key: TextDirection.Vertical, text: "Vertical" },
              { key: TextDirection.Horizontal, text: "Horizontal" },
            ]}
          />
        </div>
        <div style={{ display: "flex", gap: "10px" }}>
          <PrimaryButton onClick={runOcrClicked}>Run OCR</PrimaryButton>
        </div>
        <div />
        <div style={{ display: "flex", gap: "10px" }}>
          <PrimaryButton onClick={invokeFunction("ipc_create_window")}>
            Create new window
          </PrimaryButton>
        </div>
      </div>
    </main>
  );
}
