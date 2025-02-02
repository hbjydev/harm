import {
  Cross1Icon,
  MinusIcon,
  OpenInNewWindowIcon,
} from "@radix-ui/react-icons";

import "./App.css";
import { getCurrentWindow, Window } from "@tauri-apps/api/window";
import { PropsWithChildren, useEffect, useState } from "react";
import { Shell } from "./components/shell";
import { InitMenu } from "./components/init-menu";
import { ConfigProvider, useConfig } from "./components/config-context";
import { invoke } from "@tauri-apps/api/core";

const TitlebarButton = ({
  children,
  onClick,
}: PropsWithChildren<{
  onClick: () => void | Promise<void>;
}>) => {
  return (
    <button
      className="w-12 h-8 flex flex-col items-center justify-center hover:bg-zinc-300"
      onClick={onClick}
    >
      {children}
    </button>
  );
};

function App() {
  const [appWindow, setAppWindow] = useState<Window | undefined>(undefined);

  useEffect(() => {
    setAppWindow(getCurrentWindow());
    return () => { };
  }, []);

  const minimize = () => appWindow?.minimize();
  const maximize = () => appWindow?.toggleMaximize();
  const close = () => appWindow?.close();

  return (
    <div className="bg-zinc-100 flex flex-col min-h-screen">
      <div
        data-tauri-drag-region
        className="h-8 flex items-center justify-center pl-4 bg-zinc-900 text-white select-none"
      >
        <div
          data-tauri-drag-region
          className="flex items-center justify-between select-none w-full"
        >
          <div
            data-tauri-drag-region
            className="flex items-center gap-4 *:select-none"
          >
            <span data-tauri-drag-region className="select-none cursor-default">
              HARM
            </span>
            <span data-tauri-drag-region className="select-none cursor-default">
              v0.1.0
            </span>
          </div>
          <div className="flex items-center">
            <TitlebarButton onClick={minimize}>
              <MinusIcon />
            </TitlebarButton>
            <TitlebarButton onClick={maximize}>
              <OpenInNewWindowIcon />
            </TitlebarButton>
            <TitlebarButton onClick={close}>
              <Cross1Icon />
            </TitlebarButton>
          </div>
        </div>
      </div>

      <ConfigProvider>
        <InnerApp />
      </ConfigProvider>
    </div>
  );
}

const InnerApp = () => {
  const config = useConfig();

  useEffect(() => {
    if (config === undefined) return () => {};
    invoke("start_api");
  }, [config])

  if (!config) return null;

  if (!config.reforger_path) return <InitMenu />;

  return <Shell>Hello</Shell>;
};

export default App;
