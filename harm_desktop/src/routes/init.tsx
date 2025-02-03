import { createFileRoute, useRouter } from "@tanstack/react-router";
import { InitMenu } from "../components/init-menu";
import { useConfig } from "../lib/state/config";
import { useEffect } from "react";

export const Route = createFileRoute("/init")({
  component: RouteComponent,
});

function RouteComponent() {
  const config = useConfig();
  const router = useRouter();

  useEffect(() => {
    if (!config || !config.reforger_path) return;
    router.navigate({ href: "/" });
  }, [config]);

  return <InitMenu />;
}
