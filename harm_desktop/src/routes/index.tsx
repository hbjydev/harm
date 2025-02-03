import { createFileRoute } from "@tanstack/react-router";
import { Shell } from "../components/shell";

export const Route = createFileRoute("/")({
  component: RouteComponent,
});

function RouteComponent() {
  return <Shell>Hello "/"!</Shell>;
}
