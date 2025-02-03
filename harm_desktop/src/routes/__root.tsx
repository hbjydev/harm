import { createRootRoute, Outlet, redirect } from "@tanstack/react-router";
import {
  Cross1Icon,
  MinusIcon,
  OpenInNewWindowIcon,
} from "@radix-ui/react-icons";

import { getCurrentWindow, Window } from "@tauri-apps/api/window";
import { PropsWithChildren, useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { QueryClientProvider } from "@tanstack/react-query";
import {
  RQKEY_HARM_CONFIG,
  useConfig,
  useConfigQueryOptions,
} from "../lib/state/config";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { queryClient } from "../lib/query";
import { AppConfig } from "../lib/config";

export const Route = createRootRoute({
  component: RootComponent,
  loader: async ({ location }) => {
    await queryClient.prefetchQuery(useConfigQueryOptions());
    const config = queryClient.getQueryData<AppConfig>([RQKEY_HARM_CONFIG]);
    if (config && !config.reforger_path && location.href !== "/init") {
      throw redirect({ href: "/init" });
    }
  },
});

function RootComponent() {
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

      <QueryClientProvider client={queryClient}>
        <InnerApp>
          <Outlet />
        </InnerApp>
        <ReactQueryDevtools initialIsOpen={false} />
      </QueryClientProvider>
    </div>
  );
}

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

const InnerApp = ({ children }: PropsWithChildren) => {
  const config = useConfig();

  useEffect(() => {
    if (config === undefined || !config.reforger_path) return;
    invoke("start_api");
  }, [config]);

  return children;
};
