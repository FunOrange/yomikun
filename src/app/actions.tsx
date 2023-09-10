import { invoke } from "@tauri-apps/api/tauri";

export default function Actions() {
  const invokeFunction =
    (functionName: string, args = {}) =>
    async () => {
      const result = await invoke<string>(functionName, args);
      console.log(result);
    };

  return (
    <div style={{ display: "flex", gap: "10px" }}>
      <button onClick={invokeFunction("print_cwd")}>print_cwd</button>
      <button onClick={invokeFunction("greet", { name: "Next.js" })}>
        greet
      </button>
      <button onClick={invokeFunction("translate")}>translate</button>
    </div>
  );
}
