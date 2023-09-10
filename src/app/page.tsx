"use client";
import styles from "./page.module.css";
import { PrimaryButton } from "@fluentui/react";
import { invoke } from "@tauri-apps/api/tauri";
import { ComboBox } from "@fluentui/react";
import { useState } from "react";

export default function Home() {
  const comboId = "combo-psm";
  const [psm, setPsm] = useState(5);
  const psmOptions = [
    {
      key: 0,
      text: "0 - Orientation and script detection (OSD) only.",
    },
    { key: 1, text: "1 - Automatic page segmentation with OSD." },
    {
      key: 2,
      text: "2 - Automatic page segmentation, but no OSD, or OCR.",
    },
    {
      key: 3,
      text: "3 - Fully automatic page segmentation, but no OSD. (Default)",
    },
    {
      key: 4,
      text: "4 - Assume a single column of text of variable sizes.",
    },
    {
      key: 5,
      text: "5 - Assume a single uniform block of vertically aligned text.",
    },
    { key: 6, text: "6 - Assume a single uniform block of text." },
    { key: 7, text: "7 - Treat the image as a single text line." },
    { key: 8, text: "8 - Treat the image as a single word." },
    {
      key: 9,
      text: "9 - Treat the image as a single word in a circle.",
    },
    { key: 10, text: "10 - Treat the image as a single character." },
    {
      key: 11,
      text: "11 - Sparse text. Find as much text as possible in no particular order.",
    },
    { key: 12, text: "12 - Sparse text with OSD." },
    {
      key: 13,
      text: "13 - Raw line. Treat the image as a single text line, bypassing hacks that are Tesseract-specific.",
    },
  ];

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

  return (
    <main className={styles.main}>
      <div className={styles.description}>
        <h2>Yomikun v0.1</h2>
      </div>

      <div className={styles.center}>
        <div>
          <label id={comboId}>Page segmentation mode</label>
          <ComboBox
            aria-labelledby={comboId}
            placeholder="Select from dropdown..."
            selectedKey={psm}
            onChange={(e, option) => setPsm(option?.key as number)}
            options={psmOptions}
          />
        </div>
        <div style={{ display: "flex", gap: "10px" }}>
          <PrimaryButton onClick={invokeFunction("ipc_ocr", { psm: psm ?? 5 })}>
            Run OCR
          </PrimaryButton>
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
