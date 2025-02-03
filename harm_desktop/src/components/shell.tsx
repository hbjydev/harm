import { ChevronDownIcon, ChevronUpIcon } from "@radix-ui/react-icons";
import { PropsWithChildren, useState } from "react";
import { useGetServersQuery } from "../lib/state/servers";

type ShellProps = PropsWithChildren<{}>;

export const Shell = ({ children }: ShellProps) => {
  const serversQuery = useGetServersQuery();

  return (
    <div className="flex-1 grid grid-cols-[250px_auto]">
      <div className="flex flex-col bg-zinc-800 text-white p-4 gap-4">
        <SidebarSection title="Servers" collapsedByDefault={false}>
          A
        </SidebarSection>
        <SidebarSection title="Servers">B</SidebarSection>
        <SidebarSection title="Servers">C</SidebarSection>
        <SidebarSection title="Servers">D</SidebarSection>
      </div>
      <div className="p-4">{children}</div>
    </div>
  );
};

const SidebarSection = (
  {
    title,
    children,
    collapsedByDefault = true,
  }: PropsWithChildren<{
    title: string;
    collapsedByDefault?: boolean;
  }>,
) => {
  const [collapsed, setCollapsed] = useState(!collapsedByDefault);

  return (
    <div className="flex flex-col gap-2 last:mt-0">
      <div
        className="flex items-center justify-between cursor-pointer select-none"
        onClick={() => setCollapsed(!collapsed)}
      >
        <span className="text-medium">{title}</span>
        {collapsed ? <ChevronUpIcon /> : <ChevronDownIcon />}
      </div>
      {collapsed && (
        <div className="flex flex-col *:py-2 pl-2">
          {children}
        </div>
      )}
    </div>
  );
};
