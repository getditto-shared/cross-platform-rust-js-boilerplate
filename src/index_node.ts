export * from "./common"
import { NativeStore } from "./native";
export const open = async (): Promise<{ Store: NativeStore }> => {
  return { Store: NativeStore };
};
