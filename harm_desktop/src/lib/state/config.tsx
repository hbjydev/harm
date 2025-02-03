import {
  queryOptions,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { invoke } from "@tauri-apps/api/core";
import { AppConfig } from "../config";

export const RQKEY_HARM_CONFIG = "harm:config";
export const RQKEY_HARM_CONFIG_UPDATE = "harm:config:update";

export const useConfigQueryOptions = () =>
  queryOptions({
    queryKey: [RQKEY_HARM_CONFIG],
    queryFn: () => invoke<AppConfig>("get_config"),
  });

export const useConfigQuery = () => useQuery(useConfigQueryOptions());

export const useConfig = () => {
  const { data } = useConfigQuery();
  return data;
};

export const useUpdateConfigMutation = () => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationKey: [RQKEY_HARM_CONFIG_UPDATE],
    mutationFn: ({ config }: { config: AppConfig }) =>
      invoke<AppConfig>("update_config", { config }),
    onSuccess: async () => {
      await queryClient.invalidateQueries({ queryKey: [RQKEY_HARM_CONFIG] });
      await queryClient.refetchQueries({ queryKey: [RQKEY_HARM_CONFIG] });
    },
  });
};
