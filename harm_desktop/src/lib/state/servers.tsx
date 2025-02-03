import { queryOptions, useQuery } from "@tanstack/react-query";
import { queryClient } from "../query";
import { RQKEY_HARM_CONFIG } from "./config";
import { AppConfig } from "../config";

export const RQKEY_SERVERS = (id?: string) => `servers${id ? `:${id}` : ''}`;

export const getServersOptions = () => queryOptions({
  queryKey: [RQKEY_SERVERS()],
  queryFn: async () => {
    const config = queryClient.getQueryData<AppConfig>([RQKEY_HARM_CONFIG])!;
    const res = await fetch(`http://localhost:${config.api_port}/servers`, { mode: 'no-cors' });
    return res.json() as Promise<{ next_page: string; data: { id: string; name: string; config: Record<string, Record<string, string> | string> }[]; }>;
  },
  enabled: queryClient.getQueryData([RQKEY_HARM_CONFIG]) !== undefined,
});

export const useGetServersQuery = () => useQuery(getServersOptions());
