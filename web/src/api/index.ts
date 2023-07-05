import { useSessionInfoStore } from "@/store/session_info";
import { Configuration } from "@/swagger";

export function conf(): Configuration {
  return new Configuration({
    apiKey: useSessionInfoStore().session?.secret,
    basePath:
      process.env.NODE_ENV === "development"
        ? "http://localhost:3000"
        : window.location.origin,
  });
}
