import { useState } from "react";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { useConfig, useUpdateConfigMutation } from "../lib/state/config";

export const InitMenu = () => {
  const config = useConfig();
  const { mutate } = useUpdateConfigMutation();
  const [reforgerPath, setReforgerPath] = useState("");

  if (!config) return null;

  const onOpenReforgerBtn = async () => {
    const file = await openDialog({
      multiple: false,
      directory: false,
    });
    if (file) setReforgerPath(file);
  };

  const onSave = async () => {
    config.reforger_path = reforgerPath;
    mutate({ config });
  };

  return (
    <main className="my-auto px-6 py-4 mx-auto max-w-screen-sm bg-white shadow-md rounded-sm flex flex-col gap-4">
      <div className="flex flex-col gap-1">
        <h1 className="text-xl font-bold">Setup</h1>
        <p className="text-zinc-700">Let's get HARM configured.</p>
      </div>
      <div className="flex flex-col gap-2">
        <button
          className="px-4 py-2 bg-zinc-800 text-white rounded-md"
          onClick={onOpenReforgerBtn}
        >
          Arma Reforger Server Path
        </button>
        {reforgerPath && <p>{reforgerPath}</p>}

        <button className="px-4 py-2 bg-zinc-800 text-white rounded-md" onClick={onSave}>
          Save
        </button>
      </div>
    </main>
  );
};
