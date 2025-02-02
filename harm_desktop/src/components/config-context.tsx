import { createContext, PropsWithChildren, useContext, useEffect, useState } from "react";
import { AppConfig } from "../lib/config";
import { invoke } from "@tauri-apps/api/core";

export const AppConfigContext = createContext<AppConfig | undefined>(undefined);

export const ConfigProvider = ({ children }: PropsWithChildren) => {
  const [config, setConfig] = useState<AppConfig | undefined>(undefined);

  useEffect(() => {
    invoke<AppConfig>("get_config").then(setConfig);
    return () => { };
  }, []);

  return (
    <AppConfigContext.Provider value={config}>
      {children}
    </AppConfigContext.Provider>
  );
};

export const useConfig = () => {
  const ctx = useContext(AppConfigContext);
  return ctx;
};
